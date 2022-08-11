use std::collections::HashMap;

fn main() -> Result<(), String> {
    let numbers = vec![1, 2, 3, 4, 5, 6];
	// This thread calculates average
    let median_thread =
        std::thread::spawn(|| numbers.iter().sum::<i32>() as f32 / numbers.len() as f32);

	// This thread calculates mode
    let mode_thread = std::thread::spawn(|| {
        let mut counter = HashMap::new();

        for value in numbers.iter() {
            *counter.entry(value).or_insert(0) += 1;
        }

        counter.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
    });

	// We join both threads and get the results if successful
	let (median, mode) = match (median_thread.join(), mode_thread.join()) {
		(Ok(median), Ok(Some(mode)) )=> Ok((median, mode)),
		_ => Err("Calculations have failed".to_string())
	}?;

	println!("For the vector {numbers:?} the median is {median} and the mode is {mode}");
	Ok(())
}
