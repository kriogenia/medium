pub const MUL_IDENTITY: usize = 1;

pub fn mul<T: std::ops::Mul<Output = T>>(left: T, right: T) -> T {
	left * right
}