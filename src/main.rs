mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;

use crate::args::{Cli, Commands};
use crate::commands::{decode, encode, print, remove};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
	match &Cli::parse().command {
		None => Err(Error::from("expected a command")),

		Some(Commands::Encode {
			input_file,
			chunk_type,
			message,
			output_file,
		}) => encode(input_file, chunk_type, message, output_file),

		Some(Commands::Decode {
			input_file,
			chunk_type,
		}) => decode(input_file, chunk_type),

		Some(Commands::Remove {
			input_file,
			chunk_type,
		}) => remove(input_file, chunk_type),

		Some(Commands::Print { input_file }) => print(input_file),
	}
}
