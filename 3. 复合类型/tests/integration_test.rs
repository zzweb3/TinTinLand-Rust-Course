use course03;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count: u32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);


#[derive(Debug)]
struct ArticleModule;

#[test]
fn test() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

#[test]
fn test00() {
    let mut s1 = String::from("many");
    let s2 = s1.clone();
    s1.remove(0);
    println!("{} {}", s1, s2);
}

#[test]
    fn test0() {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count:1,
        };

        user1.email = String::from("anotheremail@example.com");

        println!("{:#?}", user1);
    }

    #[test]
    fn test1() {
        let active = true;
        let username = String::from("someusername123");
        let email = String::from("someone@example.com");
        let user1 = User{
            active,
            username,
            email,
            sign_in_count : 1,
        };

        println!("{:#?}", user1);
    }

    #[test]
    fn test2() {
        let active = true;
        let username = String::from("someusername123");
        let email = String::from("someone@example.com");
        let user1 = User {
            active,
            username,
            email,
            sign_in_count: 1
        };

        println!("user1: {:#?}", user1);

        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };

        //println!("user1: {:#?}", user1);
        println!("user2: {:#?}", user2);
    }

    #[test]
    fn test3() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 1, 0);
        let another = origin;

        println!("black: {:#?}", black);
        //println!("origin: {:#?}", origin);
        println!("another: {:#?}", another);
    }

    #[test]
    fn test4() {
        let module = ArticleModule;
        println!("module: {:?}", module);
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area1(self: Self, n: u32) -> u32 {
            self.width * self.height * n
        }

        fn area2(self: &Self, n: u32) -> u32 {
            self.width * self.height * n
        }

        fn area3(self: &mut Self, n:u32) -> u32 {
            self.width * self.height * n
        }
    }

    #[test]
    fn test5() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("The n times area of the rectangle is {} square pixels.", 
            rect1.area1(5)
        );
        //println!("The n times area of the rectangle is {} square pixels.", rect1.area2(5));
        //println!("The n times area of the rectangle is {} square pixels.", rect1.area3(5));
    }

    #[derive(Debug)]
struct Rectangle1 {
    width: u32,
    height: u32,
}

impl Rectangle1 {
    fn area(&self, n: u32) -> u32 {
        self.width * self.height * n
    }
}

impl Rectangle1 {
    fn can_hold(&self, other: &Rectangle1) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle1 {
    fn numbers (rows: u32, cols: u32) -> u32 {
        rows * cols
    } 
}

#[test]
fn test6() {
    let rect1 = Rectangle1 {
        width: 30,
        height: 50,
    };

    let r1 = &rect1;
    let r2 = &&rect1;
    let r3 = &&&&&&&&&&&&&&&&&&&&&&rect1;
    let r4 = &&r1;
    
    let r1_1 = r1.area(5);
    let r1_2 = r2.area(5);
    let r1_3 = r3.area(5);
    let r1_4 = r4.area(5);

    println!("r1_1: {}", r1_1);
    println!("r1_2: {}", r1_2);
    println!("r1_3: {}", r1_3);
    println!("r1_4: {}", r1_4);
    //
    let another = Rectangle1 {
        width: 20,
        height: 30,
    };
    let can_hole_bool = rect1.can_hold(&another);
    println!("can_hole_bool: {}", can_hole_bool);

    //
    let xx = Rectangle1::numbers(10, 10);
    println!("xx: {}", xx);
}

//--------------------Newtype模式---------------------//
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// 舍去不满一年的部分
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

#[test]
fn test6_1() {
    let age = Years(5);
    println!("{}", age.0);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}

//--------------------enum---------------------//
#[derive(Debug)]
enum WebEvent{
    //An 'enum' variant may either be 'unit-like'
    PageLoad,
    PageUnload,
    //like tuple structs
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click{ x:i64, y:i64},
}

#[test]
fn test7() {
    //
    let a = WebEvent::PageLoad;
    let b = WebEvent::PageUnload;
    let c = WebEvent::KeyPress('c');
    let d = WebEvent::Paste(String::from("batman"));
    let e = WebEvent::Click { x: 320, y: 240 };
    
    println!("WebEvent a: {:#?}", a);
    println!("WebEvent b: {:#?}", b);
    println!("WebEvent c: {:#?}", c);
    println!("WebEvent d: {:#?}", d);
    println!("WebEvent e: {:#?}", e);
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color1 {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

#[test]
fn test8() {
    let x = Number::Zero;
    println!("{}", x as i32);
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);
    
    println!("rose are #{:06x}", Color1::Red as i32);
    println!("violets are #{:06x}", Color1::Blue as i32);
    println!("violets are {:?}", Color1::Blue as i32);
}

enum Foo{}
impl Foo {
    fn run() -> String {
        String::from("空枚举")
    }
}
#[test]
fn test9() {
    //let a = Foo;
    let x = Foo::run();
    println!("{}", x)
}

enum VeryVerboseEnumOfThingsToDoWithNumbers{
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y:i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

#[test]
fn test10() {
    let add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let sum = add.run(10, 5);
    println!("sum: {}", sum);
    println!("sum1: {}", VeryVerboseEnumOfThingsToDoWithNumbers::Add.run(3, 8));

    let subtract = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;
    let result = subtract.run(10, 3);
    println!("result: {}", result);
}

#[test]
fn test11() {
    let number = 13;
    println!("Tell me about {}", number);

    match number {
        1 => println!("One!"),
        2|3|5|7|11 => println!("This is a prime"),
        13..=19=> println!("A teen"),
        _=> println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} - > {}", boolean, binary);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

#[test]
fn test12() {
    //let a = UsState::Alabama;
    let a = UsState::Alaska;
    let b = Coin::Quarter(a);
    let r = value_in_cents(b);

    println!("{}", r);
    
    //
    let us_state = UsState::Alabama;
    let coin = Coin::Quarter(us_state);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

#[test]
fn test13() {
    let mut optionl = Some(0);

    while let Some(i) = optionl {
        if i > 9 {
            println!("Greater than 9, quit!");
            optionl = None;
        } else {
            println!("'i' is '{:?}'. Try again.", i);
            optionl = Some(i + 1);
        }
    }

}

#[test]
fn test14() {
    let a = (1, 2, 'a');
    let (b, c, d) = a;

    println!("{:#?}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}

fn foo() -> (u32, u32, char) {
    (1, 2, 'a')
}
#[test]
fn test15() {
    let (b, c, d) = foo();

    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}

#[derive(Debug)]
struct User1 {
    name: String,
    age: u32,
    student: bool,
}

#[test]
fn test16() {
    let a = User1 {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    let User1 {
        name,
        age,
        student,
    } = a;
    println!("name: {}", name);
    println!("age: {}", age);
    println!("student: {}", student);
    //println!("a: {:#?}", a);
}

fn foo1((a, b, c) : (u32, u32, char)) {
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

#[test]
fn test17() {
    let a = (1, 2, 'a');
    foo1(a);
}

#[derive(Debug)]
struct User2 {
    name: String,
    age: u32,
    student: bool,
}

fn foo3(User2 {
    name,
    age,
    student,
} : User2){
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
}

#[test]
fn test18() {
    let a = User2 {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    foo3(a);
}

fn foo_1(s: &str) {}
fn foo_2(s: &[u32]) {}
#[test]
fn test19() {
    let s = String::from("aaa");
    let a = &s;
    foo_1(&s);
    foo_1("888999ddd"); //&str

    let v = vec![1,2,3,4,5];
    foo_2(&v);
    foo_2(&[2,2,2,2,2,2]);
}

#[test]
fn test20() {
    let s1 = String::from("a superman.");
    let s2 = String::from("two superman.");
    let s3 = String::from("3 superman.");
    let s4 = String::from("four superman.");

    let mut v = vec![s1, s2, s3, s4];
    //println!("{}", s1);
    println!("{:?}", v);

    //let a = v[0];   //不能move出来
    //TODO:修改数组中第一个值
    v.remove(0);
    v.insert(0, String::from("xxxx"));
    println!("{:?}", v);

    //TODO:修改数组中第一个值中的某一部分
    let mut a = &mut v[0];
    a.push_str("yyyy");
    a.remove(0);
    a.remove(0);
    a.remove(0);
    a.remove(1);
    
    println!("{:?}", v);
}

#[test]
fn test21() {
    use std::collections::HashMap;

    let mut book_reviews = HashMap::new();

    book_reviews.insert( 
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    println!("{:#?}", book_reviews);

    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.", book_reviews.len());
    }

    book_reviews.remove("The Adventures of Sherlock Holmes");

    println!("{:#?}", book_reviews);

    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{book} : {review}"),
            None => println!("{book} is unreviewed."),
        }
    }
    //TODO: 
    println!("Reivew for Jane: {}", book_reviews["Pride and Prejudice"]);

    for (book, review) in &book_reviews {
        println!("{book}: \"{review}\"")
    }
}

fn random_stat_buff() -> u8 {
    return 42;
}

#[test]
fn test22() {
    use std::collections::HashMap;

    let mut player_stats = HashMap::new();

    player_stats.entry("health").or_insert(100);

    println!("{:#?}", player_stats);

    player_stats.entry("defence").or_insert_with(random_stat_buff);

    println!("{:#?}", player_stats);

    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();

    println!("{:#?}", player_stats);
    //player_stats.entry("mana").or_insert(10);
    player_stats.entry("mana").and_modify(|mana| *mana += 200).or_insert(100);
    
    println!("{:#?}", player_stats);
}

#[test]
fn test23() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);
}

#[test]
fn test24() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    println!("{:#?}", map);

    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }

    println!("{:#?}", map);
}

#[test]
fn test25() {
    use std::collections::HashMap;

    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("c", 10),
        ("d", 9),
    ]);

    for (key, val) in map.iter() {
        println!("key: {key}, val: {val}");
    }
}

#[test]
fn test26() {
    use std::collections::HashMap;

    let mut map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);
    for (key, val) in &map {
        println!("开始=> key: {key} <-> val: {val}");
    }

    for (_, val) in map.iter_mut() {
        *val *= 2;
    }

    for (key, val) in &map {
        println!("中间=> key: {key} <-> val: {val}");
    }
    for ele in map.into_iter() {
        println!("最后=> {} <-> {}", ele.0, ele.1);
    }
}

#[test]
fn test27() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
