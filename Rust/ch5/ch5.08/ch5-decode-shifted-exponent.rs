// f32 내부 들여다보기 - 2

fn main() {
    // 지수부를 분리하기 위해 비트 이동이 2번 필요하다
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    // 우선 가수부 비트를 덮어쓰게끔 오른쪽으로 23번 시프트한다
    let exponent_ = n_bits >> 23;
    // 그런 다음 AND 마스크 연산(& 0xff)으로 부호 비트를 제외한다
    let exponent_ = exponent_ & 0xff;
    // 편차를 빼준다
    let exponent_ = (exponent_ as i32) - 127;

}