use chat_bubbles::ChatBubble;

pub fn main() {
    let ChatBubble { svg, .. } = chat_bubbles::variant1(0.0, 0.0, "#32c7f5");
    std::fs::write("test.svg", svg).unwrap();
    open::that("test.svg").unwrap();
}
