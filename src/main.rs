use clap::Parser;
use unitconvert::{
    cli::{Cli, Commands},
    convert::{
        UnitConverter, distance::DistanceConverter, get_converter, mass::MassConverter,
        temperature::TemperatureConverter,
    },
    expression::parse_expression,
};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Convert(args) => {
            println!(
                "Converting {} from {} to  {}...",
                args.value, args.from, args.to
            );
            let converter = get_converter(&args.from, &args.to).unwrap_or_else(|e| {
                println!("Error: {}", e);
                std::process::exit(1);
            });

            match converter.convert(args.value, &args.from, &args.to) {
                Ok(result) => {
                    println!(
                        "{} {} = {} {}",
                        args.value,
                        converter.get_unit_string(&args.from),
                        result,
                        converter.get_unit_string(&args.to)
                    )
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Expression(args) => match parse_expression(&args.expr) {
            Ok(expression) => match get_converter(&expression.from, &expression.to) {
                Ok(converter) => {
                    match converter.convert(expression.value, &expression.from, &expression.to) {
                        Ok(result) => println!(
                            "{} {} = {} {}",
                            expression.value,
                            converter.get_unit_string(&expression.from),
                            result,
                            converter.get_unit_string(&expression.to)
                        ),
                        Err(e) => eprintln!("Conversion error: {}", e),
                    }
                }
                Err(e) => eprintln!("Unsupported conversion: {}", e),
            },
            Err(e) => eprintln!("Failed to parse expression: {}", e),
        },
        Commands::List(args) => match &args.category {
            Some(category) => {
                println!("Listing units in category: {}", category);
                match category.as_str() {
                    "distance" | "d" => {
                        for unit in DistanceConverter.supported_units() {
                            println!("{}", unit);
                        }
                    }
                    "mass" | "m" => {
                        for unit in MassConverter.supported_units() {
                            println!("{}", unit);
                        }
                    }
                    "temperature" | "t" => {
                        for unit in TemperatureConverter.supported_units() {
                            println!("{}", unit);
                        }
                    }
                    other => {
                        eprintln!("Unknown unit type: '{}'", other);
                        std::process::exit(1);
                    }
                }
            }
            None => {
                println!("Supported unit types:");
                println!(" - distance");
                println!(" - mass");
                println!(" - temperature");
            }
        },
    }
}
