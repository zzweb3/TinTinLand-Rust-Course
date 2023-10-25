
fn foo(s: String) -> String{
    println!("{s}");
    s
}

fn foo1(s: &String) {
    println!("in fn foo: {s}");
}

fn foo2(s: &mut String) {
    s.push_str(" You are batman.");
}

#[derive(Debug)]
struct User {
    man:bool,
    age:u64,
}

/**
 * TODO: 第二课 Rust所有权-课堂笔记及思考题
 */
#[cfg(test)] // 标注测试模块
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = 10u32;
        let b = a;
        println!("{a}");
        println!("{b}");
    }

    #[test]
    #[ignore]
    fn test1() {
        let s1 = String::from("I am a superman.");
        let s2 = s1;
        //println!("{s1}");   //move
        println!("{s2}");
    }

    #[test]
    fn test2() {
        let s1 = String::from("I am a superman.");
        let s2 = s1.clone();
        println!("{s1}");
        println!("{s2}");
    }

    #[test]
    fn test3() {
         let s1 = String::from("I am a superman.");
         let s1 = foo(s1);
         println!("{s1}");
    }

    fn extend_tup(mut tup : (i32, f64, char, &str)) -> (i32, f64, char, &str) {
        tup.1 = 3.1415f64;
        tup.3 = "uuuu";
        tup
    } 

    #[test]
    fn test4() {
        //TODO: 不可变引用是copy的
        let tup= (10, 30.2, '9', "xxx");
        let tup_1 = extend_tup(tup);
        println!("tup: {:?}", tup);
        println!("tup_1: {:?}", tup_1);

        let ss = "xxx555果果";
        let ss1 = ss;
        println!("ss: {}", ss);
        println!("ss1: {}", ss1);
    }

    #[test]
    fn test5() {
        let u = User{
            man:true,
            age:10
        };
        let u1 = u;
        println!("{:?}", u1);
        //println!("{:?}", u);  //TODO: 结构体中的变量即使都是copy操作的，如u32，bool 等，但是结构体还是存放在heap中，struct 默认值move操作
    }

    #[test]
    fn test6() {
        let a = 10u32;
        let b = &a;
        let c = &&&&&a;
        let d = &b;
        let e = b;  //b和e都是对a的引用，由于引用是固定长度的值，做的就引用的复制操作，而并没有对a的值10u32再复制一份
        println!("{a}");
        println!("{b}");
        println!("{c}");
        println!("{d}");
        println!("{e}");
    }

    #[test]
    fn test7() {
        let s1 = String::from("I am a superman.");
        let s2 = &s1;
        let s3 = &&&&&s1;
        let s4 = &s2;
        let s5 = s2;
        println!("{s1}");
        println!("{s2}");
        println!("{s3}");
        println!("{s4}");
        println!("{s5}");
    }

    #[test]
    fn test8() {
        let mut a = 10u32;
        let b = &mut a;
        *b = 20;

        println!("{b}");
        println!("{a}");
    }

    #[test]
    #[ignore]
    fn test9() {
        let mut a = 10u32;
        let b = &mut a;
        *b = 20;

        //println!("{a}"); TODO: immutable borrow occurs here
        println!("{b}");
    }

    #[test]
    fn test10() {
        let mut a = 10u32;
        let b = &mut a;
        *b = 20;
        let c = &a;
    }

    #[test]
    #[ignore]
    fn test11() {
        let mut a = 10u32;
        let b = &mut a;
        *b = 20;
        let c = &a;

        //println!("{b}");
    }

    #[test]
    #[ignore]
    fn test12() {
        let mut a = 10u32;
        let b = &mut a;
        *b = 20;
        let c = &a;

        //println!("{c}");
    }

    #[test]
    #[ignore]
    fn test13() {
        let mut a = 10u32;
        let c = &a;
        let b = &mut a;
        *b = 20;

        //println!("{c}");
    }

    #[test]
    fn test14() {
        let mut a = 10u32;
        let c = &a;
        let b = &mut a;
        *b = 20;

        println!("{b}");
    }

    #[test]
    #[ignore]
    fn test15() {
        let mut a = 10u32;
        let b = &mut a;
        *b = 20;
        let d = &mut a;
        
        //println!("{b}");
    }

    #[test]
    #[ignore]
    fn test16() {
        let mut a = 10u32;
        let r1 = &mut a;
        let r2 = r1;

        //println!("{r1}");
    }

    #[test]
    #[ignore]
    fn test17() {
        let mut a = 10u32;
        let r1 = &mut a;
        let r2 = r1;

        println!("{r2}");
    }

    #[test]
    #[ignore]
    fn test18() {
        let mut a = 10u32;
        let r1 = &mut a;
        let r2 = r1;    //move

        //println!("{a}");
        println!("{r2}");
    }

    #[test]
    fn test19() {
        let s1 = String::from("I am a superman.");
        foo1(&s1);
        println!("{s1}");
    }

    #[test]
    fn test20() {
        let mut s1 = String::from("I am a superman.");
        println!("{s1}");
        foo2(&mut s1);
        println!("{s1}");
    }

}
