use clap::{AppSettings, Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "car")]
#[clap(about = "A CLI to train a self driving car", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Converts the binary file created in the Arduino SD card to a CSV text file, ready to be read.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Convert {
        /// The binary file to convert to csv
        file: String,
    },
}
