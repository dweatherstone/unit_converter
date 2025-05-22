use clap::{Args, Parser, Subcommand};

/// Simple unit converter
#[derive(Parser)]
#[command(name = "unitconvert")]
#[command(author = "David Weatherstone")]
#[command(version = "0.1.0")]
#[command(about = "Convert between different unit types", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Convert(ConvertArgs),
    List(ListArgs),
}

#[derive(Args)]
pub struct ConvertArgs {
    #[arg(help = "Value to convert")]
    pub value: f64,

    #[arg(short, long, help = "Source unit (e.g. m, ft, kg)")]
    pub from: String,

    #[arg(short, long, help = "Target unit (e.g. m, ft, kg)")]
    pub to: String,
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long)]
    pub category: Option<String>,
}
