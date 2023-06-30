# Chat Bubbles

Usage:

```rs
let text_width: f32 = 50.0;
let text_height: f32 = 75.0;
let svg: String = chat_bubbles::create(text_width, text_height);
```

Notes:

- Due to padding and rounding currently being hardcoded, overlay your text relative the center of the svg's canvas.

![Example Bubble](example.svg)
