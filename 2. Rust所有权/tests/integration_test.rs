
#[test]
fn test() {
    //如果引入reborrow的概念
    //1. ‘&T’ 和 '&mut T' 可以同时存在
    let mut n = 10_i32;
    let p1 = &mut n;
    let p2 :&i32 = p1;  //必须有类型提示符
    println!("{}", p2);
    println!("{}", p1);
}

#[test]
fn test1() {
    //2. '&mut T' 可以存在多个
    let mut n = 10_i32;
    let p1 = &mut n;
    //let p2 :&mut i32 = p1;  //必须有类型提示符
    let p2 :&mut i32 = &mut *p1;  //编译器会先解引用，再借用，即: p1 --> &mut *p1
    println!("{}", p2);
    println!("{}", p1);
}

//出得课堂题目

#[test]
fn test2() {
    //实现swap(&mut a, &mut b)方法体：交换a，b中的值，最终 a = 6，b =5；
    let mut a = 5;
    let mut b = 6;

    swap(&mut a, &mut b);   //实现swap方法
    println!("after swap(..), a={}", a);
    println!("after swap(..), b={}", b);
}

pub fn swap(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

#[test]
fn test3() {
    let ss = "xxx555果果";
    let ss1 = ss;
    println!("ss: {}", ss);
    println!("ss1: {}", ss1);
}

#[test]
fn test4() {
    let tup= (10, 30.2, '9', "xxx");
    let tup_1 = extend_tup(tup);
    println!("tup: {:?}", tup);
    println!("tup_1: {:?}", tup_1);
}

fn extend_tup(mut tup : (i32, f64, char, &str)) -> (i32, f64, char, &str) {
    tup.1 = 3.1415_f64;
    tup.3 = "한국어Korean";
    tup
} 

#[test]
fn test5() {
    let a = -4;
    let b = a % 64;
    println!("-5%64 = {}", b);
    println!("-5%64 = {}", -4%64);
}


#[test]
fn test6() {
    let tup_1= (10, 30.2, 'A', "I am Superman!");
    let tup_2 = extend_tup1(tup_1);
    println!("tup_1: {:?}", tup_1);
    println!("tup_2: {:?}", tup_2);
}

fn extend_tup1(mut tup : (i32, f64, char, &str)) -> (i32, f64, char, &str) {
    tup.0 = -100_i32;
    tup.1 = 3.141592_f64;
    tup.3 = "한국어!";
    tup
} 