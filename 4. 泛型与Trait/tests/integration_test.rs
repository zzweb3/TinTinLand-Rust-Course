//use course04;

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
       if item > largest {
        largest = item;
       }
    }
    largest
} 

fn largest_char(list: Vec<char>) -> char {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[test]
fn test0() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(char_list);
    println!("The largest char is {}", result);
}


fn largest<T: std::cmp::PartialOrd + Copy> (list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[test]
fn test1() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(char_list);
    println!("The largest char is {}", result);
}


fn largest_1<T: std::cmp::PartialOrd> (list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    &largest
}

#[test]
fn test2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_1(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_1(&char_list);
    println!("The largest char is {}", result);
}

#[derive(Debug)]
struct Point<T> {
    x:T,
    y:T
}

#[test]
fn test3() {
    let integer = Point {x:5, y:10};
    let float = Point {x:1.0, y:4.0};

    println!("integer => {:?}", integer);
    println!("float => {:?}", float);
}

#[derive(Debug)]
struct Point1<T, U> {
    x:T,
    y:U,
}
#[test]
fn test4() {
    let a = Point1 {x:5, y:1.0};
    let b = Point1 {x:1.0, y: 'A'};

    println!("a => {:?}", a);
    println!("b => {:?}", b);
}

#[derive(Debug)]
struct Point3<T> {
    x:T,
    y:T
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point3<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[test]
fn test5() {
    let p = Point3 {x:10, y:15};
    println!("p.x() => {}", p.x());

    let p = Point3 {x:10.3, y:15.1};
    println!("{}", p.distance_from_origin());
}

#[derive(Debug)]
struct Point4<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point4<X1,Y1> {
    //TODO: 方法也是可以单独定义泛型的
    fn mixup<X2,Y2>(self, other: Point4<X2, Y2>) -> Point4::<X1, Y2> {
        Point4 { 
            x: self.x, 
            y: other.y 
        }
    }
}

#[test]
fn test6() {
    let p = Point4{x: 5, y: 20.5};
    let o = Point4{x: 'A', y:"good"};
    let mix = p.mixup(o);
    println!("mix => {:?}", mix);
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self { x, y}
    }
}

impl<T> Pair<T> 
where 
    T: std::fmt::Display + std::cmp::PartialOrd
{
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[test]
fn test7() {
    let p = Pair::new(55, 10);
    p.cmp_display();
}

trait Atr {
    fn foo(&self);
}

struct Foo;

impl Atr for Foo {
    fn foo(&self) {
        println!("666");
    }
}

#[test]
fn test8() {
    let foo = Foo;
    foo.foo();
}

trait StreamingIterator {
    type Item;
}

#[derive(Debug)]
struct Foo1<T>
    where T: StreamingIterator<Item=String> 
{
    x: T
}

#[derive(Debug)]
struct A;
impl StreamingIterator for A {
    type Item = String;
}

#[test]
fn test9() {
    //TODO: Foo1::<A>
    let a = Foo1::<A> {
        x: A
    };
    println!("{:#?}", a);
}

trait Animal {
    fn talk(&self);
}

struct Cat {}
struct Dog {}

impl Animal for Cat {
    fn talk(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn talk(&self) {
        println!("bark");
    }
}

fn animal_talk (a: &dyn Animal) {
    a.talk();
}

fn animal_talk1<T: Animal>(a: &T) {
    a.talk();
}

fn animal_talk2 (a: Box<dyn Animal>) {
    a.talk();
}

#[test]
fn test10() {
    let d = Dog{};
    let c = Cat{};
    animal_talk(&d);
    animal_talk(&c);

    animal_talk(&d);
    animal_talk(&c);

    let ans: Vec<&dyn Animal> = vec![&d, &c];
    //println!("{:#?}", ans);

    animal_talk2(Box::new(d));
    animal_talk2(Box::new(c));
}

//------------
trait Atr1 {
    fn xxx(self);
    fn foo(&self);
}

struct Foo2;

impl Atr1 for Foo2 {
    fn xxx(self) {
        println!("xxx");
    }
    fn foo(&self) {
        println!("666");
    }
}

#[test]
fn test10_1() {
    let foo2 = Foo2;
    foo2.foo();
    foo2.xxx();
}
//-----------------

trait Animal1 {
    fn nop(&self) {
        println!("888");
    }
}

struct Cat1 {}
impl Animal1 for Cat1 {}

#[test]
fn test11() {
    let c = Cat1{};
    //let ct: &dyn Animal1 = &c;
    c.nop();
}

struct Sheep {}
struct Cow {}

trait Animal2 {
    fn noise(&self) -> &'static str;
}

impl Animal2 for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal2 for Cow {
    fn noise(&self) -> &'static str {
        "mooooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal2> {
    if random_number < 0.5 {
        Box::new(Sheep{})
    } else {
        Box::new(Cow{})
    }
}

#[test]
fn test12() {
    //let random_number = 0.234;
    let random_number = 1.234;
    let animal = random_animal(random_number);
    println!("You have randomly chosen an animal, and it says {}", animal.noise());
}

#[test]
fn test13() {
    let a = "xxxx";
    let aa = &a;
    let ref aaa = a;
    println!("{}", aaa);
}
//-------------test14--------------//
use std::io::Read;
use std::io::Result as IOResult;
struct  Foo3 {
    name: String
}

struct  Bar {
    age: u8
}

impl Read for Foo3 {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        //self.read(buf)
        IOResult::Ok(888)
    }
}

impl Read for Bar {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        IOResult::Ok(666)
    }
}

enum Wrapper {
    Foo3ccc(Foo3),
    Barccc(Bar),
}

impl Read for Wrapper {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        match *self {
            Wrapper::Foo3ccc(ref mut foo) => foo.read(buf),
            Wrapper::Barccc(ref mut bar) => bar.read(buf),
        }
    }
}

#[test]
fn test14() {
    let mut v: Vec<u8> = vec![1,2,3,4,5];
    let bar = Bar {
        age:20,
    };
    let bar_read = Wrapper::Barccc(bar).read(&mut v);
    println!("bar_read => {:#?}", bar_read);
    //
    let foo3 = Foo3 {
        name: "foo3".to_string(),
    };
    let foo3_read = Wrapper::Foo3ccc(foo3).read(&mut v);
    println!("foo3_read => {:#?}", foo3_read);

}


