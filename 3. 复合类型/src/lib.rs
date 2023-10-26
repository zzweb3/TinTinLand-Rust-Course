
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count: u32,
}

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

        user1.email = String::from("anotheremail@example.com");

        println!("{:#?}", user1);
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

        println!("{:#?}", user1);
    }

}
