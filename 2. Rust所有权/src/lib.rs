

/**
 * TODO: 第二课 Rust所有权-课堂笔记及思考题
 */
#[cfg(test)] // 标注测试模块
mod tests {

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
        
    }
}
