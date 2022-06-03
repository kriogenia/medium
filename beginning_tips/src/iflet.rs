fn overflow(n: i32, max: i32) -> Option<i32> {
    if n > max {
        Some(n - max)
    } else {
        None
    }
}

fn main() {
    let n = 15;
	// instead of this
	match overflow(n, 10) {
		Some(overflow) => println!("The input value has gone {overflow} over the maximum"),
		_ => {}
	}
	// do this
    if let Some(overflow) = overflow(n, 10) {
        println!("The input value has gone {overflow} over the maximum")
    }
}
