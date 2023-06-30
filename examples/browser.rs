pub fn main() {
    let svg = chat_bubbles::create(100.0, 50.0);
    std::fs::write("test.svg", svg).unwrap();
    open::with("test.svg", "inkscape").unwrap();
}
