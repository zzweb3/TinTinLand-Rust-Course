

#[derive(Debug)]
struct User {
    active: bool,
    userName: String,
    email: String,
    age:u64,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

/**
 * TODO: 第一课 初认Rust-课堂笔记及思考题
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        print!("hello world.");
    }

    #[test]
    fn test1() {
        let a = 1;
        println!("hello world. {}", a);
        println!("hello world. {a}");
        //
        let a = 2u32;
        println!("hello world. {}", a);
        //
        let a:u32 = 3;
        println!("hello world. {}", a);
    }

    #[test]
    fn test2() {
        //let a = 10_000_000_000u32;    //超出长度
        let a:u64 = 10_000_000_000u64;
        println!("hello world. {}", a);
        // TODO: 大数相乘
        let a:u64 = u64::MAX;
        let b:u64 = u64::MAX;
        let c:u128 = u128::from(a) * u128::from(b);
        println!("hello world. {c}");
    
    }

    #[test]
    fn test3() {
        let a = 'A';
        println!("hello world. {}", a);
        //
        let a = b'A';
        println!("hello world. {}", a);

        //TODO: 输入emoji命令：CTRL + CMD + Space
        let a = '🤡';
        println!("hello world. {}", a);
    }

    #[test]
    fn test4() {
        let a = 10_000_000.0f32;
        println!("hello world. {}", a);

        let a = 10_000_000.;
        println!("hello world. {}", a);

        let a = 10_000_000.0;
        println!("hello world. {}", a);
        let a = 10_000_000.0f64;
        println!("hello world. {}", a);
    }

    #[test]
    fn test5() {
        //TODO: char 4个字节，32位
        let c = 'z';
        let z:char = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("{}", heart_eyed_cat);
    }

    #[test]
    fn test6() {
        //TODO: String 存储UTF8编码
        let s = String::from("initial contents");
        let hello = String::from("عليكم السلام");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        //
    }

    #[test]
    fn test7() {
        /*
         * 如何从字符串提取出’你‘：
         */
        let hello = String::from("你好");
        let char_vec :Vec<char> = hello.chars().collect::<Vec<_>>();
        println!("hello world. {}", char_vec[0]);

        /*
         * 如何将字符串转换成字符数据：
         * String -> [char] -> Vec<char>
         */
        //TODO: 实现不好！！！
        let hello1 = String::from("안녕하세요");
        let char_size = hello1.as_str().chars().count();
        assert_eq!(char_size, 5);
        let mut chars = ['0'; 5];
        let char_itr = hello1.as_str().chars();
        let mut i = 0;
        char_itr.for_each(|c| {
            chars[i] = c;
            i += 1;
        });
        println!("{:?}", chars);
        //改进后
        let hello2 = String::from("안녕하세요");
        let char_vec2 = hello2.as_str().chars().collect::<Vec<char>>();
        let chars :&[char] = &char_vec2;    //TODO: !!!!!!!!!!再研究研究
        println!("{:?}", chars);
    }

    #[test]
    fn test8() {
        let s = r##"ab"c"def"##;
        println!("hello world. {}", s);
    }

    #[test]
    fn test9() {
        //字面量
        let bytestring = b"this is a byte string";
        println!("A byte string : {:?}", bytestring);

        let escaped = b"\x52\x75\x73\x74 as bytes";
        println!("Some escaped bytes : {:?}", escaped);

        //TODO: ？？？？？
        let raw_bytestring = br"\u{211D} is not escaped here";
        println!("raw_bytestring: {:#?}", raw_bytestring);

        let _quotes = br#"You can also use "fancier" formatting, \like with normal raw strings"#;
        println!("_quotes: {:?}", _quotes);

        let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb";    // "ようこそ" in SHIFT-JIS
        println!("shift_jis: {:?}", shift_jis);

    }

    #[test]
    fn test10() {
        //数组 - 定长数组
        let a : [i32; 5] = [1, 2, 3, 4, 5];
        let a = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
        println!("months: {:#?}", months);
    }

    #[test]
    fn test11() {
        let a = [1, 2, 3, 4, 5];
        let b = a[0];   //TODO: 因为静态数组是创建在栈中，因为静态数组是固定的，所以，此处知识简单数据copy，不是move
        println!("{:?}", b);
        println!("{:?}", a);
        println!("{:?}", a[0]);
    }

    #[test]
    fn test12() {
        //let v = Vec::new();
        //let v = vec![1, 2, 3];
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("{:#?}", v);
        println!("{:?}", v[3]);
        //println!("{:?}", v[4]);
    }

    #[test]
    fn test13() {
        let v : Vec<u32> = vec![1, 2, 3, 4, 5];
        println!("{:?}", v);
        let t = v[5];
        println!("{:?}", t);
    }

    #[test]
    fn test14() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:#?}", scores);
    }

    #[test]
    fn test15() {
        //元组中可以放不同类型
        let tup = (500, 6.4, 1);
        println!("{:?}", tup);
        println!("{}, {}, {}",tup.0, tup.1, tup.2);

        let x = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;
        println!("{:?}", x);
        println!("{}, {}, {}", five_hundred, six_point_four, one);

        let x = (500, 6.4, 1, 'A');
        println!("{:?}", x);
    }

    #[test]
    fn test16() {
        let u = User{
            active: true,
            userName: String::from("mike"),
            email: String::from("zz@gmail.com"),
            age: 30,
        };

        println!("{:#?}", u);
    }

    #[test]
    fn test17() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        println!("{:?}, {:?}", four, six);
    }

    #[test]
    fn test18() {
        let number = 6;
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4,3,2");
        }
    }

    #[test]
    fn test19() {
        //TODO: if else 是可以返回值的
        let x = 1;
        let y = if x == 0 {
            100
        } else {
            101
        };
        println!("y is {}", y)
    }

    #[test]
    fn test20() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 20;
            }
        };
        println!("The result is {result}");
    }

    #[test]
    fn test21() {
        // while true {} 等同于 loop {}， 编译器会提示使用loop，效率更高。

        let mut number = 3;
        while number != 0 {
            println!("{number} !");
            number -=1;
        }
        println!("LIFTOFF!!!");
    }

    #[test]
    fn test22() {
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is : {element}");
        }
    }

    #[test]
    fn test23() {
        //range用法:https://doc.rust-lang.org/std/ops/struct.Range.html
        //左闭右开
        for number in 1..4 {
            println!("{number}");
        }
        println!("++++++++++++++++++");
        //左闭右闭
        for number in 1..=4 {
            println!("{number}");
        }
        println!("++++++++++++++++++");
        //
        for number in (1..4).rev() {
            println!("{number}");
        }
    }

    #[test]
    fn test24() {
        for ch in '1' ..= 'z' {
            println!("{ch}");
        }
    }


    fn print_a_b(value:i32, unit_label: char) {
        println!("The value of a b is: {value}, {unit_label}");
        println!("The value of a b is: {} {}", value, unit_label);
    }

    #[test]
    fn test25() {
        print_a_b(3, 'Z');
    }
}
