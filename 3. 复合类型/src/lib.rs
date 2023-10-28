#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count: u32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

/**
 * TODO: 第三课 复合类型-课堂笔记及思考题
 */
#[cfg(test)] // 标注测试模块
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count:1,
        };
        println!("user1更新前=> {:#?}", user1);
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






    #[test]
    fn test2222() {
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
    fn test3333() {
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
