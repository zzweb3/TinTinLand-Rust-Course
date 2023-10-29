pub mod level_2;
pub use level_2::level_2_mod;

pub mod level_1_mod {
    pub fn print_a_to_Z() {
        println!("一层子模块,循环打印从’a’~’Z’ 之间的所有字符:");
        for element in ('Z'..='a').rev() {
            print!("{}", element)
        }
        println!();
    }
}