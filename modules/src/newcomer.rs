pub mod mul;
mod strings;	// Hidden to the outside world

pub fn hello() -> &'static str {
	strings::HELLO
}