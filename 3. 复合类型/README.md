# TinTinLand-Rust-Course
> TinTinLand Rust 课程笔记-复合类型
> 
> 课件ppt：https://drive.google.com/file/d/1FhpFDCWFJ1uivmZWXu1TgpmJ6m-zFeHB/view

# 作业点评会 & Office Hour
目录 ?????????

## 复合类型
> 复合类型也叫组合类型, Rust中的复合类型可以分为两大类；
>> 1. 结构体(structure type): 多个类型组合在一起共同表达一个值的复杂数据结构
>> 2. 枚举(enum type), 即标签联合(tagged union),也叫不相交并集(disjoint union), 可以存储一组不同但固定的类型中的某个类型的对象, 具体是哪个类型由其标签决定。

## 1、结构体(structure type)

### 1.1 使用结构体组织关联数据

<details> <summary>结构体定义</summary>

```rust
//结构体定义
struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count: u32,
}

//结构体更新
#[test]
fn test0() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count:1,
    };
    println!("user1更新前 => {:#?}", user1);
    //更新
    user1.email = String::from("anotheremail@example.com");
    println!("user1更新后 => {:#?}", user1);
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
    println!("user1 => {:#?}", user1);
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

    println!("user1 => {:#?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    //println!("user1 => {:#?}", user1); //TODO: user1 已被借用
    println!("user2 => {:#?}", user2);
}
```

</details>






### 1.2 元组结构体（tuple structs）
所谓元组结构体,也就是元组和结构体的结合体, 元组结构体有类型名,但是无字段名,也即字段是匿名的。

#### 元组结构体-例子
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[test]
fn test3() {
    let black = Color(0, 0, 0);
    let mut origin = Point(0, 1, 2);

    println!("black => {:#?}", black);
    println!("origin => {:#?}", origin);

    println!("orgin修改前: origin.0 => {}, origin.1 => {}, , origin.2 => {}", origin.0, origin.1, origin.2);
    origin.0 = 9;
    origin.1 = 8;
    origin.2 = 7;
    println!("orgin修改后: origin.0 => {}, origin.1 => {}, , origin.2 => {}", origin.0, origin.1, origin.2);
}
```

### 1.3 单元结构体（unit-like structs）
单元结构体就是只有一个类型名字,没有任何字段的结构体。定义和创建实例时连后面的花括号都可以省略。
如果想要在某种类型上实现trait功能，但不需要在类型中存储数据时使用。
TODO: 待补充案例！！！
#### 单元结构体-例子
```rust
struct ArticleModule;

fn main() {
    let module = ArticleModule;
}


```



## 2、枚举(enum type)


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

答案

#[test]
fn test() {
    let s1 = String::from("a superman.");
    let s2 = String::from("two superman.");
    let s3 = String::from("3 superman.");
    let s4 = String::from("four superman.");

    let mut v = vec![s1, s2, s3, s4];
    println!("{:?}", v);

    //TODO:修改数组中第1个值
    v.remove(0);
    v.insert(0, String::from("I work hard to learn rust."));
    println!("{:?}", v);
}

#[test]
fn test1() {
    let s1 = String::from("a superman.");
    let s2 = String::from("two superman.");
    let s3 = String::from("3 superman.");
    let s4 = String::from("four superman.");

    let mut v = vec![s1, s2, s3, s4];
    println!("{:?}", v);

    //TODO:修改数组中第1个值中的部分字符
    let mut a = &mut v[0];
    a.remove(a.len()-1);
    a.push_str(" who saves the world.");
    println!("{:?}", v);
}

```
