const TOKEN_AS_STR: &'static str = include_str!("./token.txt");
const TOKEN_AS_BYTES: &[u8] = include_bytes!("./token.txt");

fn main() {
	assert_eq!("HelLoWoRLd", TOKEN_AS_STR);
	assert_eq!("HelLoWoRLd".as_bytes(), TOKEN_AS_BYTES);
}