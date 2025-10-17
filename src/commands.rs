use std::path::PathBuf;

use crate::Result;

pub fn encode(
	input_file: &PathBuf,
	chunk_type: &str,
	message: &str,
	output_file: &Option<PathBuf>,
) -> Result<()> {
	println!(
		"{:?} {} {} {:?}",
		input_file, chunk_type, message, output_file
	);

	Ok(())
}

pub fn decode(input_file: &PathBuf, chunk_type: &str) -> Result<()> {
	println!("{:?} {}", input_file, chunk_type);

	Ok(())
}

pub fn remove(input_file: &PathBuf, chunk_type: &str) -> Result<()> {
	println!("{:?} {}", input_file, chunk_type);

	Ok(())
}

pub fn print(input_file: &PathBuf) -> Result<()> {
	println!("{:?}", input_file);

	Ok(())
}
