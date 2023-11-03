# Rust所有权

## 题目1: 下面程序执行后输出什么结果？
```rust
#[test]
fn test() {
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
```
请选择答案：
> A: 
> 编译错误！
 
> B:
```rust
tup_1: (10, 30.2, 'A', "I am Superman!")
tup_2: (10, 30.2, 'A', "I am Superman!")
```

> C:
```rust
tup_1: (10, 30.2, 'A', "I am Superman!")
tup_2: (-100, 3.141592, 'A', "한국어!")
```

> D:
```rust
tup_1: (10, 30.2, 'A', "I am Superman!")
tup_2: (-100, 3.141592, 'A', "I am Superman!")
```

>>> 正确答案：C 

___

## 题目2: 下面程序执行后输出什么结果？
```rust
#[test]
fn test() {
    let s = String::from("你好中国");
    let s1 = &s[0..2];
    println!("s1: {}", s1);
}
```
请选择答案：

> A: 
> 编译错误！
 
> B:
```rust
s1: 你
```

> C:
```rust
s1: 你好
```

>>> 正确答案：A 

___

## 题目3: 下面程序执行后输出什么结果？
```rust
#[test]
fn test() {
    let message = String::from("Hello");
    let slice = &message[2..4];

    move_me(message);

    println!("slice: {}", slice);
}

fn move_me(val: String) { }
```

请选择答案：

> A: 
> 编译错误！
 
> B:
```rust
slice: llo
```

> C:
```rust
slice: ll
```

>>> 正确答案：A 


___

## 题目4: 下面程序执行后输出什么结果？
```rust
#[test]
fn test2() {
    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;

    println!("before c: {}", c);

    let e = &&100;
    c = e;

    println!("a: {}", a);
    println!("after c: {}", c);
}
```

请选择答案：

> A: 
> 编译错误！
 
> B:
```rust
before c: 10
a: 10
after c: 10
```

> C:
```rust
before c: 10
a: 10
after c: 100
```

>>> 正确答案：C 

 
>考察点对内存的理解（选择题比较难体现出来）
>
![题目4的内存草图 图标](https://github.com/guozhouwei/TinTinLand-Rust-Course/blob/main/2.%20Rust%E6%89%80%E6%9C%89%E6%9D%83/images/exercise01.jpeg "内存草图")
讲解视频：https://www.bilibili.com/video/BV1Np4y1P78z?p=28&vd_source=c591c8fa7c4af6fd760fcdf31da64702


