use attribute_macros::HelloMacro;

trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes; // 自动生成 impl HelloMacro for Pancakes

fn main() {
    Pancakes::hello_macro(); // 输出: "Hello, Macro! My name is Pancakes!"
}