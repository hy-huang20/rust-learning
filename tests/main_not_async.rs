use embassy_executor::{Spawner, main};


/// 
/// 运行：cargo test --test main_not_async
/// 
/// 预期结果：编译错误，提示 main function must be async
/// 
/// 测试在 wsl 上执行时是否确实会在编译期执行到
/// embassy-executor-macros/src/lib.rs 中 main_std 函数
/// 
/// 如果执行 src/lib.rs 中 main_std 函数则一定会执行 src/macros/main.rs 
/// 中的 run 函数，而 run 函数针对一些非法情况有相应的输出，输出在编译错误中
/// 
/// 可以直接在 main_std 函数开头使用 println! 输出自定义内容以验证
/// 比如修改 src/lib.rs 中的 main_std 为：
/// 
/// #[proc_macro_attribute]
/// pub fn main_std(args: TokenStream, item: TokenStream) -> TokenStream {
///    println!("main std");
///    main::run(args.into(), item.into(), &main::ARCH_STD).into()
///}
/// 
/// 上面的 println!("main std") 行为手动添加
/// 
#[main]
#[test]
fn main(_spawner: Spawner) {
    // 函数体置空
}