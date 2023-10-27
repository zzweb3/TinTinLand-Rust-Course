# TinTinLand-Rust-Course
TinTinLand Rust 课程笔记-复合类型

## 课件ppt：
https://drive.google.com/file/d/1FhpFDCWFJ1uivmZWXu1TgpmJ6m-zFeHB/view




## 练习题目
```rust
/*
* 题目一：(如果下面代码片段中有不符合题意的情况，可微调)
* 1. 修改数组中第1个值，如：
* ["a superman.", "two superman.", "3 superman.", "four superman."]
* -->
* ["I work hard to learn rust.", "two superman.", "3 superman.", "four superman."]
*
* 2. 修改数组中第1个值中的部分字符，如：
* ["a superman.", "two superman.", "3 superman.", "four superman."]
* -->
* ["a superman who saves the world.", "two superman.", "3 superman.", "four superman."]
*/
fn foo() {
    let s1 = String::from("a superman.");
    let s2 = String::from("two superman.");
    let s3 = String::from("3 superman.");
    let s4 = String::from("four superman.");

    let v = vec![s1, s2, s3, s4];
    println!("{:?}", v);

}
```
