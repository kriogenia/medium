fn main() {
	let vec = vec!["f", "o", "o"];
	// instead of this
	for i in 0..vec.len() {
		println!("The character at {i} is {}", vec[i]);
	}
	// you can use this
	for (i, char) in vec.iter().enumerate() {
		println!("The character at {i} is {char}");
	}
}
