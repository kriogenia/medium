use std::ops::Add;

use num::Zero;
use vector2d::Vector2D;

fn sum_no_generic(numbers: &[i32]) -> i32 {
    let mut total = numbers[0];
    for i in numbers.into_iter().skip(1) {
        total += *i;
    }
    total
}

fn sum_generic<T>(numbers: &[T]) -> T
where
    T: Add<Output = T> + Copy,
{
    let mut total = numbers[0];
    for i in numbers.into_iter().skip(1) {
        total = total + *i;
    }
    total
}

fn sum_from_zero<T>(numbers: &[T]) -> T
where
    T: Add<Output = T> + Copy + Zero,
{
    let mut total: T = T::zero();
    for i in numbers.into_iter() {
        total = total + *i;
    }
    total
}

fn main() {
    let ints = vec![23, -4, 3, 10];
    let floats = vec![23.0, -4.0, 3.0, 10.0];
	let vectors = vec![Vector2D::new(23, -4), Vector2D::new(3, 10), Vector2D::new(1, 1)];

    assert_eq!(sum_no_generic(&ints), 32);

    assert_eq!(sum_no_generic(&ints), 32);
    assert_eq!(sum_generic(&floats), 32.0);
	assert_eq!(sum_generic(&vectors), Vector2D::new(27, 7));

    assert_eq!(sum_from_zero(&ints), 32);
    assert_eq!(sum_from_zero(&floats), 32.0);
}
