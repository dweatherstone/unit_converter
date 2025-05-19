use clap::Parser;

/// Simple unit converter (currently supports meters and feet)
#[derive(Parser, Debug)]
#[command(name = "unitconvert")]
#[command(author = "David Weatherstone")]
#[command(version = "0.1.0")]
#[command(about = "Convert between units like meters and feet", long_about=None)]
pub struct Cli {
    // The value to convert
    pub value: f64,

    // Source unit (e.g., "m" or "ft")
    #[arg(short, long)]
    pub from: String,

    // Target unit (e.g., "m" or "ft")
    #[arg(short, long)]
    pub to: String,
}
