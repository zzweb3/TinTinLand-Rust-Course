use std::marker::PhantomData;

// 定义一个包含泛型参数 T 的结构体
struct Repeater<T> {
    phantom: PhantomData<T>,
}
impl<T> Repeater<T> {
    fn new() -> Self {
        Repeater {
            phantom: PhantomData,
        }
    }
    //复读，传入什么就传出什么
    fn repeater_value(&self, s: T) -> T {
        s
    }
}

#[test]
fn test() {
    let r1 = Repeater::<String>::new();
    let b1 = r1.repeater_value("hello rust.".to_string());
    println!("{}", b1);

    let r2 = Repeater::<u8>::new();
    let b2 = r2.repeater_value(128);
    println!("{}", b2);
}
