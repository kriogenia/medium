fn main() {
    // Result of the different times when 255 + 1 overflows the u8
    assert_eq!(u8::MAX.wrapping_add(1), 0);
    assert_eq!(u8::MAX.checked_add(1), None);
    assert_eq!(u8::MAX.overflowing_add(1), (0, true));
    assert_eq!(u8::MAX.saturating_add(1), u8::MAX);
    
    // Wrapping examples
    assert_eq!(i32::MAX.wrapping_add(12), i32::MIN + 11);
    assert_eq!(0u8.wrapping_sub(1), u8::MAX);

    // Checked examples
    assert_eq!(i32::MAX.checked_pow(2), None);
    assert_eq!(1u8.checked_mul(5), Some(5));

    // Overflowing examples
    assert_eq!(i32::MIN.overflowing_abs(), (i32::MIN, true));
    assert_eq!(4i32.overflowing_div(2), (2, false));

    // Saturating examples
    assert_eq!(1u8.saturating_sub(3), 0);
    assert_eq!(2i32.saturating_neg(), -2);
}
