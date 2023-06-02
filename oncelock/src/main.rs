use std::sync::OnceLock;

static ADDRESSES: OnceLock<Vec<&str>> = OnceLock::new(); 

fn main() {
	assert!(ADDRESSES.get().is_none());

	let first_attempt = ADDRESSES.set(Vec::from([
		"http://validaddress.com",
		"http://mainmirror.com",
		"http://128.0.0.1:1337"
	]));
	assert!(first_attempt.is_ok());

	check_addresses();

	// the initial value can't be overriden
	let second_attempt = ADDRESSES.set(Vec::new());
	assert!(second_attempt.is_err_and(|v| v.is_empty()))	// is_err_and is also a new addition of Rust 1.70

}

fn check_addresses() {
	assert!(ADDRESSES.get().is_some());
    assert_eq!("http://validaddress.com", ADDRESSES.get().unwrap()[0]);
	println!("The address list is readable and correct: {:?}", ADDRESSES.get().unwrap())
}