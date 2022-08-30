pub(crate) const fn i128_shr(num: i128, shift: i32) -> i128 {
    if shift >= 0 {
        num >> shift
    } else {
        num << (-shift)
    }
}
