pub mod application;
pub mod domain;
pub mod infrastructure;

use crate::domain::services::FileBinaryConverter;
use crate::infrastructure::FileBinaryReader;

use crate::domain::model::DataRecord;
use crate::domain::ports::incoming::BinaryConverter;

use crate::application::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Convert { file } => {
            let binary_reader = FileBinaryReader::new(file);
            let binary_converter = FileBinaryConverter::new(binary_reader);
            let c = binary_converter.convert::<DataRecord>();

            if let Ok(vector) = c {
                for i in vector.iter() {
                    println!("> {:?}", i);
                }
            }
        }
    }
}
