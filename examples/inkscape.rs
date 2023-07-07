use chat_bubbles::ChatBubble;

pub fn main() {
    let ChatBubble { svg, .. } = chat_bubbles::create(184.0, 78.0);
    std::fs::write("test.svg", svg).unwrap();
    open::with("test.svg", "firefox").unwrap();
}
