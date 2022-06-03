enum ReadingError {
	OutOfBounds,
	InvalidBuffer
}

trait InsteadOfThis {
	fn next_char() -> Result<u8, ReadingError>;
	fn next_line() -> Result<String, ReadingError>;
	fn char_at(pos: usize) -> Result<u8, ReadingError>;
}

// new result declaring the error just once
type ReadingResult<T> = Result<T, ReadingError>;

trait YouCanDoThis {
	fn next_char() -> ReadingResult<u8>;
	fn next_line() -> ReadingResult<String>;
	fn char_at(pos: usize) -> ReadingResult<u8>;
}

fn get_first(buffer: &[u8]) -> ReadingResult<String> {
	if buffer.len() > 0 {
		Ok(buffer[0].to_string())
	} else {
		Err(ReadingError::OutOfBounds)
	}
}

fn main() {
    if let Ok(first) = get_first(&"foo".as_bytes()) {
		println!("The first char is {}", first)
	}
}
