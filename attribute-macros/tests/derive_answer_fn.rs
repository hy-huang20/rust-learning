use attribute_macros::AnswerFn;

#[derive(AnswerFn)]
struct Struct; // AnswerFn 没有对 Struct 做什么，只是简单地添加了一个函数 answer()

fn main() {
    assert_eq!(42, answer());
}