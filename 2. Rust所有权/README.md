# TinTinLand-Rust-Course
TinTinLand Rust 课程笔记-Rust所有权

## 课件：
https://drive.google.com/file/d/1WvBBL6thFnqUbHoatjPFdinnL0v7RbLF/view

## 课堂笔记
#### 所有权
Rust明确了所有权的概念。值也可以叫资源。所有权就是对资源拥有的权利。Rust
基于所有权定义出发,推导了整个世界。所有权的基础是三条定义:
● Rust中,每一个值(资源)都有一个所有者;
● 任何一个时刻,一个值只有一个所有者;
● 当所有者所在作用域结束的时候,值会被释放掉。

三个规则,涉及两个概念,所有者,作用域。
所谓所有者,在代码中,就表示为变量。也就是说所有者会用变量名来表示。
变量的作用域,就是变量有效(valid)的那个区间。在Rust中,简单来说就是在一
对花括号括起的里面部分中,从变量创建时开始,到花括号结束的地方。比如:
```rust
fn main() {
    let s = String::from("hello");
    // do stuff with s
}
fn main() {
    let a = 1u32;
    {
        let s = String::from("hello");
    }
    // other stuff
}
```

