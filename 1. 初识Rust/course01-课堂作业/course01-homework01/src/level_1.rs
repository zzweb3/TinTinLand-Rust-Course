pub mod level_2;

pub fn print_a_to_Z() {
    println!("一层子模块,循环打印从’a’~’Z’ 之间的所有字符:");
    for element in ('Z'..='a').rev() {
        print!("{}", element)
    }
    println!();
}

pub mod just_test_mod {
    pub fn just_test_innermod_fun() {
        println!("just_test_innermod_fun");
    }
}