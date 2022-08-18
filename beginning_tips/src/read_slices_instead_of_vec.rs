fn instead_of_this(chars: &Vec<u8>) -> Option<&u8> {
	chars.iter().find(|&&x| x == b"\n"[0])
}

fn do_this(chars: &[u8]) -> Option<&u8> {
	chars.iter().find(|&&x| x == b"\n"[0])
}

fn main() {
	let vec: Vec<u8> = vec![77, 78, 2];
	let arr: &[u8] = b"ab\n";
	let str: &str = "ab\n";

	let _ok = instead_of_this(&vec);
	//let mismatched_types = instead_of_this(&arr);
	//let mismatched_types = instead_of_this(str.as_bytes());

	let _from_vec = do_this(&vec);
	let _from_arr = do_this(arr);
	let _from_str = do_this(str.as_bytes());
}