pub const fn i128_shl(num: i128, shift: i32) -> i128 {
    if shift == 0 {
        num
    } else if shift > 0 {
        num << shift
    } else {
        num >> (-shift)
    }
}
