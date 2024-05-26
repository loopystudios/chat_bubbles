use chat_bubbles::ChatBubble;

pub fn main() {
    let ChatBubble { svg, .. } = chat_bubbles::create(100.0, 50.0, "#32c7f5", "#86e2ff");
    std::fs::write("test.svg", svg).unwrap();
    open::with("test.svg", "inkscape").unwrap();
}
