mod mul;			// Now it's hidden to
pub use mul::mul;	// But we can still access this

mod strings;	// Hidden to the outside world

pub fn hello() -> &'static str {
	strings::HELLO
}