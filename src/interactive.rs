use std::io::{self, Write};

use crate::{convert::get_converter, error::ConvertError, expression::parse_expression};

pub fn run_interactive() -> Result<(), ConvertError> {
    println!("🔁 Welcome to the Unit Converter! Type 'quit' to exit.");
    println!("You can enter expressions (e.g. '10C -> F') or type 'guided' for step-by-step mode.");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("quit") {
            println!("👋 Goodbye!");
            break;
        } else if trimmed.eq_ignore_ascii_case("guided") {
            match guided_prompt() {
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    continue;
                }
            }
        } else {
            match parse_expression(trimmed) {
                Ok(expression) => {
                    match run_conversion(expression.value, &expression.from, &expression.to) {
                        Ok(_) => continue,
                        Err(e) => eprintln!("❌ Something went wrong! The error was: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Failed to parse expression: {}", e),
            }
        }
    }
    Ok(())
}

fn guided_prompt() -> Result<(), ConvertError> {
    let value = prompt("Enter value to convert:")?
        .parse::<f64>()
        .map_err(|_| ConvertError::ParseError("Invalid number".to_string()))?;
    let from = prompt("Enter FROM unit (e.g. m, kg, C,...):")?;
    let to = prompt("Enter TO unit (e.g. m, kg, C,...):")?;

    run_conversion(value, &from, &to)?;
    Ok(())
}

fn prompt(msg: &str) -> Result<String, ConvertError> {
    print!("{} ", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn run_conversion(value: f64, from: &str, to: &str) -> Result<(), ConvertError> {
    let converter = get_converter(from, to)?;
    let result = converter.convert(value, from, to)?;
    println!(
        "✅ {} {} = {} {}",
        value,
        converter.get_unit_string(from),
        result,
        converter.get_unit_string(to),
    );
    Ok(())
}
