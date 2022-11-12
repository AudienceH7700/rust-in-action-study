fn use_value_primitive(_val: i32){
}

fn use_value_not_primitive(_val: Demo){}

struct Demo {
    a : i32,
}

fn main() {
    let a = 123;
    use_value_primitive(a);

    println!("{}", a); // 컴파일 오류 없음

    let demo = Demo {a : 123};
    use_value_not_primitive(demo);

    println!("{}", demo.a); // 컴파일 오류 발생
}