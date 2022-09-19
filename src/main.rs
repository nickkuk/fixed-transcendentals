use std::io::Write;

fn i4_is_negative(x: u8) -> bool {
    x >= 8
}

fn i4_sign_extend(x: u8) -> u8 {
    if !i4_is_negative(x) {
        x
    } else {
        0xf0 | x
    }
}

fn wide_mul_ii4(x: u8, y: u8) -> u8 {
    let u = x * y;
    0
}

fn main() {
    let f = std::fs::File::create("target/muls.txt").unwrap();
    let mut f = std::io::BufWriter::new(f);
    for x in 0u8..=0xf {
        for y in 0u8..=x {
            let ix = i4_sign_extend(x);
            let iy = i4_sign_extend(y);
            let u = x * y;
            let ui = ix.wrapping_mul(iy);
            if i4_is_negative(x) || i4_is_negative(y) {
                // if i4_is_negative(x) ^ i4_is_negative(y) {
                writeln!(f, "{x:x}  {y:x}  {u:02x}  {ui:02x}").unwrap();
            }
        }
    }
}
