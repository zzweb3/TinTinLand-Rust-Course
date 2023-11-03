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


