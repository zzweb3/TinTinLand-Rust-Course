trait A {

}

struct A1;
struct A2;
struct A3;
impl A for A1{}
impl A for A2{}
impl A for A3{}

#[derive(Debug)]
enum Wrapper {
    Type1(A),
    Type2(String),
    Type3(f64),
}

fn call_methods(wrapper: &Wrapper) {
    match wrapper {
        Wrapper::Type1(x) => println!("调用 Type1 的方法，x = {}", x),
        Wrapper::Type2(x) => println!("调用 Type2 的方法，x = {}", x),
        Wrapper::Type3(x) => println!("调用 Type3 的方法，x = {}", x),
    }
}

#[test]
fn test() {
    let vec = vec![
        Wrapper::Type1(10),
        Wrapper::Type2("hello".to_string()),
        Wrapper::Type3(3.1415926),
    ];

    for wrapper in vec {
        call_methods(&wrapper);
    }
}