
pub mod level_2_mod {
    pub fn print_A_to_z() {
        println!("二层子模块,循环打印从’A’~’z’ 之间的所有字符:");
        for element in 'A'..='z' {
            print!("{}", element)
        }
        println!();
    }
}
