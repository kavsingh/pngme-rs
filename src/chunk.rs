use std::fmt::Display;

use crate::chunk_type::ChunkType;

pub struct Chunk {
	chunk_type: ChunkType,
	data: Vec<u8>,
	length: u32,
	crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
	type Error = crate::Error;

	fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
		let len = match input[0..4] {
			[a, b, c, d] => u32::from_be_bytes([a, b, c, d]),
			_ => return Err(Self::Error::from("invalid length part")),
		};

		let chunk_type = match input[4..8] {
			[a, b, c, d] => ChunkType::try_from([a, b, c, d])?,
			_ => return Err(Self::Error::from("invalid chunk type part")),
		};

		let data = input[8..(8 + len as usize)].to_vec();

		if data.len() as u32 != len {
			return Err(Self::Error::from("invalid data part"));
		}

		let chunk = Chunk::new(chunk_type, data);
		let crc = match input[(8 + len as usize)..(12 + len as usize)] {
			[a, b, c, d] => u32::from_be_bytes([a, b, c, d]),
			_ => return Err(Self::Error::from("invalid crc part")),
		};

		if crc != chunk.crc() {
			return Err(Self::Error::from("crc mismatch"));
		}

		Ok(chunk)
	}
}

impl Display for Chunk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.data_as_string() {
			Ok(s) => f.write_str(&s),
			Err(_) => f.write_str("<invalid data>"),
		}
	}
}

impl Chunk {
	const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

	pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
		let length = data.len() as u32;
		let mut crc_bytes = chunk_type.bytes().to_vec();

		crc_bytes.append(&mut data.to_owned());

		Self {
			chunk_type,
			data,
			length,
			crc: Self::CRC.checksum(&crc_bytes),
		}
	}

	pub fn length(&self) -> u32 {
		self.length
	}

	pub fn chunk_type(&self) -> &ChunkType {
		&self.chunk_type
	}

	pub fn data(&self) -> &[u8] {
		&self.data
	}

	pub fn crc(&self) -> u32 {
		self.crc
	}

	pub fn data_as_string(&self) -> crate::Result<String> {
		match str::from_utf8(self.data()) {
			Ok(s) => Ok(s.to_string()),
			Err(_) => Err(crate::Error::from("failed to convert")),
		}
	}

	pub fn as_bytes(&self) -> Vec<u8> {
		self.length
			.to_be_bytes()
			.iter()
			.chain(self.chunk_type.bytes().iter())
			.chain(self.data.iter())
			.chain(self.crc.to_be_bytes().iter())
			.copied()
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::*;
	use crate::chunk_type::ChunkType;

	fn testing_chunk() -> Chunk {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656334;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		Chunk::try_from(chunk_data.as_ref()).unwrap()
	}

	#[test]
	fn test_new_chunk() {
		let chunk_type = ChunkType::from_str("RuSt").unwrap();
		let data = "This is where your secret message will be!"
			.as_bytes()
			.to_vec();
		let chunk = Chunk::new(chunk_type, data);
		assert_eq!(chunk.length(), 42);
		assert_eq!(chunk.crc(), 2882656334);
	}

	#[test]
	fn test_chunk_length() {
		let chunk = testing_chunk();
		assert_eq!(chunk.length(), 42);
	}

	#[test]
	fn test_chunk_type() {
		let chunk = testing_chunk();
		assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
	}

	#[test]
	fn test_chunk_string() {
		let chunk = testing_chunk();
		let chunk_string = chunk.data_as_string().unwrap();
		let expected_chunk_string = String::from("This is where your secret message will be!");
		assert_eq!(chunk_string, expected_chunk_string);
	}

	#[test]
	fn test_chunk_crc() {
		let chunk = testing_chunk();
		assert_eq!(chunk.crc(), 2882656334);
	}

	#[test]
	fn test_valid_chunk_from_bytes() {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656334;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

		let chunk_string = chunk.data_as_string().unwrap();
		let expected_chunk_string = String::from("This is where your secret message will be!");

		assert_eq!(chunk.length(), 42);
		assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
		assert_eq!(chunk_string, expected_chunk_string);
		assert_eq!(chunk.crc(), 2882656334);
	}

	#[test]
	fn test_invalid_chunk_from_bytes() {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656333;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		let chunk = Chunk::try_from(chunk_data.as_ref());

		assert!(chunk.is_err());
	}

	#[test]
	pub fn test_chunk_trait_impls() {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656334;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

		let _chunk_string = format!("{}", chunk);
	}
}
