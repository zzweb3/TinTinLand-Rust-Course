
mod level_1;
use level_1::level_1_mod as mod1;
use level_1::level_2::level_2_mod as mod2;

/**
 * 创建一个Rust工程
 *（1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
 *（2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
 *（3）使用Cargo编译运行此工程
 *（需要自行发现其中的细节，一个考点是：ascii码字符的顺序）
 */
 fn main() {
    mod1::print_a_to_Z();
    mod2::print_A_to_z();
}