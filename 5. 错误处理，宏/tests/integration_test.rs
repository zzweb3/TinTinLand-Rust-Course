//use course05;


//-----------------01-----------------//
static ASTRING: &'static str = "abc";

fn foo() -> &'static str {
    ASTRING
}

#[test]
fn test01() {
    let s = foo();
}
//-----------------02-----------------//
fn foo1<'a> (s: &'a str) -> &'a str {
    if s.len() > 5 {
        &s[..5]
    } else {
        s
    }
}

#[test]
fn test02() {
    let s = String::from("abcdef");
    println!("{}", foo1(&s));
}
//------------------03----------------//
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[test]
fn test03() {
    let config_max = Some(3_8);
    match config_max {
        Some(max) => println!("The maximum is cofigured to be {}", max),
        _ => (),
    }

    let config_max = Some(3_u8);
    if let Some(max) = config_max {
        println!("The maximum is cofigured to be {}", max);
    }

    let a = UsState::Alabama;
    let coin = Coin::Quarter(a);

    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

}

//------------------04----------------//
#[test]
fn test04() {
    let mut x = Some("value");
    //x = None;
    println!("x =>{}", x.expect("fruits are healthy"));
    assert_eq!(x.expect("fruits are healthy"), "value");
}

//------------------05----------------//
#[test]
fn test05() {
    let text = Some("Hello, world!".to_string());
    let text_length = text.as_ref().map(|s| s.len());
    println!("still can print text : {text:?}");
    println!("still can print text : {text_length:?}");
}

//------------------06----------------//
#[test]
fn test06() {
    use std::fs::File;

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("文件未找到！Problem opening the file: {:?}", error),
    };
}

//------------------07----------------//
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("/Users/zhouzhou/rust_workspace/TinTinLand-Rust-Course/5. 错误处理，宏/tests/hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),    //主要要return
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

#[test]
fn test07() {
        let content = read_username_from_file();
        match content {
            Ok(c) => println!("文件内容：{}", c),
            Err(e) => println!("文件异常：{}", e.to_string()),
        }       
}

//------------------08----------------//
fn read_username_from_file1() -> Result<String, io::Error> {
    let mut username_file = File::open("/Users/zhouzhou/rust_workspace/TinTinLand-Rust-Course/5. 错误处理，宏/tests/hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[test]
fn test08() {
    let content = read_username_from_file1();
    match content {
        Ok(c) => println!("文件内容：{}", c),
        Err(e) => println!("文件异常：{}", e.to_string()),
    }  
}

//------------------09----------------//
fn fn1() -> Result<(), std::io::Error> {
    let greeting_file = File::open("/Users/zhouzhou/rust_workspace/TinTinLand-Rust-Course/5. 错误处理，宏/tests/hello.txt")?;
    //let a = "100".parse::<u32>()?;
    Ok(())
}

fn fn2() -> Result<String, std::io::Error> {
    let mut username_file = File::open("/Users/zhouzhou/rust_workspace/TinTinLand-Rust-Course/5. 错误处理，宏/tests/hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    //let a = "100".parse::<u32>()?;
    let a = "100".parse::<u32>().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?;

    Ok(username)
}
#[test]
fn test09() {
    //如何变更一个ERROR
    let a = "100".parse::<u32>().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"));
    //
    let content = fn2();
    match content {
        Ok(c) => println!("文件内容：{}", c),
        Err(e) => println!("文件异常：{}", e.to_string()),
    } 
}

//------------------10----------------//
fn last_char_of_first_line(text: &str) ->Option<char> {
    text.lines().next()?.chars().last()
}
#[test]
fn test10() {
    let text = "000w
    123
    345
    567
    ";
    println!("{:?}", last_char_of_first_line(text));
}

//------------------11----------------//
macro_rules! add {
    ($a:expr, $b:expr) => {
        {
            $a + $b
        }
    };
}

#[test]
fn test11() {
    add!(1, 2);
}
//TODO: 执行命令：rustc +nightly -Zunpretty=expanded macro.rs

//------------------12----------------//
macro_rules! add {
    ($a:expr, $b:expr) => {
        {
            $a + $b
        }
    };
    ($a:expr) => {
        $a
    }
}

#[test]
fn test12() {
    let x = 10;
    add!(1, 2);
    add!(x);
}
//------------------13----------------//
macro_rules! add_as {
    ($a:expr, $b:expr, $typ:ty) => {
        {
            $a as $typ + $b as $typ
        }
    }
}

#[test]
fn test13() {
    println!("{}", add_as!(1, 2, u8));
}
//------------------14----------------//
macro_rules! add_as {
    ($($a:expr), *) => {
        {
            0
            $(+$a)*
        }
    }
}

#[test]
fn test14() {
    println!("{}", add_as!(1, 2, 3,4,5));
}
//------------------15----------------//
macro_rules! calculate {
    (eval $e:expr) => {
        let val: usize = $e;
        println!("{} = {}", stringify!{$e}, val);
    }
}

#[test]
fn test15() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (5 / 3)
    }
}
//------------------16----------------//