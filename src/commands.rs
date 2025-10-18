use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::Result;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(
	input_file: &PathBuf,
	chunk_type: &str,
	message: &str,
	output_file: &Option<PathBuf>,
) -> Result<()> {
	let out_path = output_file.to_owned().unwrap_or_else(|| {
		let mut out = input_file.clone();

		out.set_file_name("output.png");

		out
	});

	let data = fs::read(input_file)?;
	let chunk_t = ChunkType::from_str(chunk_type)?;
	let chunk = Chunk::new(chunk_t, message.bytes().collect::<Vec<u8>>());
	let mut png = Png::try_from(data.as_ref())?;

	png.append_chunk(chunk);
	fs::write(out_path, png.as_bytes())?;

	Ok(())
}

pub fn decode(input_file: &PathBuf, chunk_type: &str) -> Result<()> {
	let data = fs::read(input_file)?;
	let chunk_data = Png::try_from(data.as_ref())?
		.chunk_by_type(chunk_type)
		.map(|c| c.data_as_string());

	match chunk_data {
		Some(Ok(content)) => println!("{content}"),
		Some(Err(_)) => println!("could not decode content for {chunk_type}"),
		None => println!("could not find {chunk_type}"),
	}

	Ok(())
}

pub fn remove(input_file: &PathBuf, chunk_type: &str) -> Result<()> {
	let data = fs::read(input_file)?;
	let mut png = Png::try_from(data.as_ref())?;

	png.remove_first_chunk(chunk_type)?;
	fs::write(input_file, png.as_bytes())?;

	Ok(())
}

pub fn print(input_file: &PathBuf) -> Result<()> {
	let data = fs::read(input_file)?;
	let png = Png::try_from(data.as_ref())?;

	println!("{}", png);

	Ok(())
}
