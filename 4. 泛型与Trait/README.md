# TinTinLand-Rust-Course
TinTinLand Rust 课程笔记-泛型与Trait

## 笔记：
https://drive.google.com/file/d/1MHGA5W1_T9OW_zMUBPFcW2DM2wRkRt9O/view

## 课堂练习题

### 题目1: 下面程序执行结果是什么？
```rust
    trait Animal {
        fn talk(self);
    }

    struct Cat;
    impl Animal for Cat {
        fn talk(self) {
            println!("Meow!");
        }
    }

    struct Dog;
    impl Animal for Dog {
        fn talk(self) {
            println!("Bark!");
        }
    }

    fn animal_talk(a: dyn Animal) {
        a.talk();
    }


    #[test]
    fn test() {
        let c = Cat{};
        let d = Dog{};

        animal_talk(c);
        animal_talk(d);
    }
```
> >答案：
>
> A 
> 
>   编译错误
> 
> B 
> 
>   Meow!
> 
>   Bark!
> 
> C 
> 
>   运行错误
>
>
> > 正确答案：A
> 
> 原因: &dyn Animal，不能获取所有权

### 题目2: 下面程序执行结果是什么？
```rust
    trait Animal {
        fn talk(&self);
    }

    #[derive(Debug)]
    struct Cat;
    impl Animal for Cat {
        fn talk(&self) {
            println!("Meow!");
        }
    }

    #[derive(Debug)]
    struct Dog;
    impl Animal for Dog {
        fn talk(&self) {
            println!("Bark!");
        }
    }

    fn animal_talk(a: Box<dyn Animal>) {
        a.talk();
    }


    #[test]
    fn test() {
        let c = Cat{};
        let d = Dog{};

        animal_talk(Box::new(c));
        animal_talk(Box::new(d));

        println!("{:#?}", c);
        println!("{:#?}", d);

    }
```

>> 答案：
>
> A 
> 
>   编译错误
>
> 
> B 
> 
>   Meow!
> 
>   Bark!
> 
> C 
> 
>   运行错误
>
>
> 正确答案：A 
> 
> 原因: Box::new()可以获取所有权