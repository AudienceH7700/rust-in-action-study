// f32 내부 들여다보기 - 1

fn main(){
    // 비트 부호를 분리시키기 위해 비트를 오른쪽으로 31번 이동한다.
    let n: f32 = 42.42;
    let n_bits = n.to_bits();
    let sign_bit = n_bits >> 31;

    println!("{}", sign_bit);
}