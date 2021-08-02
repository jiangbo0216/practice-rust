// ? 文件名不能和文件夹名字相同
// 没有参数, 没有返回
// main 是每个 Rust 可执行程序最先运行的代码
// 使用 rustc ./hello.rs 编译文件, 只适合简单的 Rust 程序, (ahead-of-time), 在windows上, 会生成一个 .pdb 文件, 包含调试信息
fn main () {
  // cp print! 是 Rust macro (宏), 区别函数(没有!)
  // 双引号是字符串, 作为 print! 的参数
  // 以 ; 表示结尾
  print!("hello world!");
}