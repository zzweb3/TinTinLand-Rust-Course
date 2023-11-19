
trait AAA {
    type Mytype;
}

struct MyStruct;

impl AAA for MyStruct {
    type Mytype = i32;
}

#[test]
fn test() {
    let value: <MyStruct as AAA>::Mytype = 42;
    // 在这里，MyStruct::Mytype 将是 i32 类型
    println!("{}", value);
}
