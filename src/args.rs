use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
	Encode {
		input_file: PathBuf,
		chunk_type: String,
		message: String,

		#[arg(short, long)]
		output_file: Option<PathBuf>,
	},

	Decode {
		input_file: PathBuf,
		chunk_type: String,
	},

	Remove {
		input_file: PathBuf,
		chunk_type: String,
	},

	Print {
		input_file: PathBuf,
	},
}
