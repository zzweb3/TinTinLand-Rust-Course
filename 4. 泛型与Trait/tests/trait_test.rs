//use course04;

/*
    TODO: rust的格式化trait很有意思，非常巧妙，推荐看源码
*/

#[test]
fn test0() {
    let x = 42; // 42 是 '101010' 二进制

    assert_eq!(format!("{x:b}"), "101010");
    assert_eq!(format!("{x:#b}"), "0b101010");

    assert_eq!(format!("{:b}", -16), "11111111111111111111111111110000");
}

#[test]
fn test01() {
    use std::fmt;

    struct Length(i32);

    impl fmt::Binary for Length {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let val = self.0;

            let a = fmt::Binary::fmt(&val, f); // 委托给 i32 的实现
            println!("xxx");
            a
        }
    }

    let l = Length(107);

    assert_eq!(format!("l as binary is: {l:b}"), "l as binary is: 1101011");

    assert_eq!(
        format!("l as binary is: {l:#032b}"),
        "l as binary is: 0b000000000000000000000001101011"
    );
}