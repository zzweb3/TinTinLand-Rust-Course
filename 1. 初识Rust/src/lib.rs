

/**
 * TODO: 课堂笔记及思考题
 */
#[cfg(test)]
mod tests {

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

}
