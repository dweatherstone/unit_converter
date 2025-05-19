use clap::Parser;
use unitconvert::{cli::Cli, convert::get_converter};

fn main() {
    let args = Cli::parse();

    let converter = get_converter(&args.from, &args.to).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    match converter.convert(args.value, &args.from, &args.to) {
        Ok(result) => println!(
            "{} {} = {} {}",
            args.value,
            converter.get_unit_string(&args.from),
            result,
            converter.get_unit_string(&args.to)
        ),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
