/**
 * TODO: 第三课 复合类型-课堂笔记及思考题
 */
#[cfg(test)] // 标注测试模块
mod tests {
    use super::*;

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

}
