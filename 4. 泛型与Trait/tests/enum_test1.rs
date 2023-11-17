trait Greeting {
    fn greeting(&self) -> &str;
}

struct Cat;
impl Greeting for Cat {
    fn greeting(&self) -> &str {
        "Meow!"
    }
}

struct Dog;
impl Greeting for Dog {
    fn greeting(&self) -> &str {
        "Woof!"
    }
}

//impl 做函数参数
fn print_greeting_impl(g: impl Greeting) {
    println!("{}", g.greeting());
}

//impl 做函数返回值，下面代码会编译报错
fn return_greeting_impl(i: i32) -> impl Greeting {
    if i > 10 {
        return Cat;
    }
    Dog
}


#[test]
fn test() {
    let c = Cat{};
    let d = Dog{};

    print_greeting_impl(c);
    print_greeting_impl(d);

}
