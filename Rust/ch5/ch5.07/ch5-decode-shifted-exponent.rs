fn main() {
    let n: f32 = 42.42;
    let n_bits = n.to_bits();
    let exponent_ = n_bits >> 23;
    let exponent_ = exponent & 0xff;
    let exponent_ = (exponent_ as i32) - 127;

}