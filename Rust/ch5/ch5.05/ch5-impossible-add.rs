#[allow(arithmetic_overflow)]

fn main() {
    let (a,b) = (200, 200);
    let c:u8 = a + b;
    println!("200 + 200 = {}", c);

    // overflow 없이 강제 실행하기 위한 실행 명령어
    // rustc -O ch5-impossible-add.rs ; ch5-impossible-add`
    // ABI Application Binary Interface
}