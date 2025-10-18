use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct ChunkType {
	bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
	type Error = crate::Error;

	fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
		if bytes.iter().any(|b| !b.is_ascii_alphabetic()) {
			return Err(Self::Error::from("invalid input"));
		}

		Ok(Self { bytes })
	}
}

impl FromStr for ChunkType {
	type Err = crate::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.as_bytes() {
			[a, b, c, d] => Self::try_from([*a, *b, *c, *d]),
			_ => Err(Self::Err::from("input must have length 4")),
		}
	}
}

impl PartialEq for ChunkType {
	fn eq(&self, other: &Self) -> bool {
		self.bytes == other.bytes
	}
}

impl Display for ChunkType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match std::str::from_utf8(&self.bytes) {
			Ok(s) => f.write_str(s),
			Err(_) => f.write_str("<invalid bytes>"),
		}
	}
}

#[allow(dead_code)]
impl ChunkType {
	pub fn bytes(&self) -> [u8; 4] {
		self.bytes
	}

	pub fn is_critical(&self) -> bool {
		self.bytes[0].is_ascii_uppercase()
	}

	pub fn is_public(&self) -> bool {
		self.bytes[1].is_ascii_uppercase()
	}

	pub fn is_reserved_bit_valid(&self) -> bool {
		self.bytes[2].is_ascii_uppercase()
	}

	pub fn is_safe_to_copy(&self) -> bool {
		self.bytes[3].is_ascii_lowercase()
	}

	pub fn is_valid(&self) -> bool {
		if self.bytes.iter().any(|b| !b.is_ascii_alphabetic()) {
			return false;
		}

		self.is_reserved_bit_valid()
	}
}

#[cfg(test)]
mod tests {
	use std::convert::TryFrom;
	use std::str::FromStr;

	use super::*;

	#[test]
	pub fn test_chunk_type_from_bytes() {
		let expected = [82, 117, 83, 116];
		let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

		assert_eq!(expected, actual.bytes());
	}

	#[test]
	pub fn test_chunk_type_from_str() {
		let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
		let actual = ChunkType::from_str("RuSt").unwrap();
		assert_eq!(expected, actual);
	}

	#[test]
	pub fn test_chunk_type_is_critical() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_critical());
	}

	#[test]
	pub fn test_chunk_type_is_not_critical() {
		let chunk = ChunkType::from_str("ruSt").unwrap();
		assert!(!chunk.is_critical());
	}

	#[test]
	pub fn test_chunk_type_is_public() {
		let chunk = ChunkType::from_str("RUSt").unwrap();
		assert!(chunk.is_public());
	}

	#[test]
	pub fn test_chunk_type_is_not_public() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(!chunk.is_public());
	}

	#[test]
	pub fn test_chunk_type_is_reserved_bit_valid() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_reserved_bit_valid());
	}

	#[test]
	pub fn test_chunk_type_is_reserved_bit_invalid() {
		let chunk = ChunkType::from_str("Rust").unwrap();
		assert!(!chunk.is_reserved_bit_valid());
	}

	#[test]
	pub fn test_chunk_type_is_safe_to_copy() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_safe_to_copy());
	}

	#[test]
	pub fn test_chunk_type_is_unsafe_to_copy() {
		let chunk = ChunkType::from_str("RuST").unwrap();
		assert!(!chunk.is_safe_to_copy());
	}

	#[test]
	pub fn test_valid_chunk_is_valid() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_valid());
	}

	#[test]
	pub fn test_invalid_chunk_is_valid() {
		let chunk = ChunkType::from_str("Rust").unwrap();
		assert!(!chunk.is_valid());

		let chunk = ChunkType::from_str("Ru1t");
		assert!(chunk.is_err());
	}

	#[test]
	pub fn test_chunk_type_string() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert_eq!(&chunk.to_string(), "RuSt");
	}

	#[test]
	pub fn test_chunk_type_trait_impls() {
		let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
		let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
		let _chunk_string = format!("{}", chunk_type_1);
		let _are_chunks_equal = chunk_type_1 == chunk_type_2;
	}
}
