use tera::{Context, Tera};

const SVG: &str = include_str!("bubble.svg.j2");

/// Color of the text bubble
const COLOR: &str = "#32c7f5";

/// Highlight color of the text bubble
const HIGHLIGHT_COLOR: &str = "#86e2ff";

/// Opacity of the chat bubble
const OPACITY: &str = "80%";

// How much rounding to apply to the shark fin
const SHARK_FIN_ROUNDING: f32 = 5.0; //px

/// How wide the shark fin is
const SHARK_FIN_WIDTH: f32 = 35.0; // px

/// How tall the shark fin is
const SHARK_FIN_HEIGHT: f32 = 10.0; // px

const HIGHLIGHT_WIDTH: f32 = 20.0; // px

// Empty space on all sides of the text
const PADDING: f32 = 30.0; // px

// Rounding needs to be smaller than padding
const ROUNDING: f32 = 30.0; // px

pub struct ChatBubble {
    /// The SVG contents
    pub svg: String,
    /// The width of the resulting viewbox
    pub vb_width: f32,
    /// The height of the resulting viewbox
    pub vb_height: f32,
}

/// Create a chat bubble
pub fn create(inner_width: f32, inner_height: f32) -> ChatBubble {
    let inner_width = inner_width.max(SHARK_FIN_WIDTH);
    let inner_height = inner_height.max(PADDING.max(ROUNDING));

    let width = PADDING.max(ROUNDING) * 2.0 + inner_width;
    let height = PADDING.max(ROUNDING) * 2.0 + inner_height;

    let fill_width = inner_width + PADDING.max(ROUNDING) * 2.0 - PADDING.min(ROUNDING) * 2.0;
    let fill_height = inner_height + PADDING.max(ROUNDING) * 2.0 - PADDING.min(ROUNDING) * 2.0;

    // Create template
    let mut tera = Tera::default();
    tera.add_raw_template("bubble", SVG).unwrap();

    let mut context = Context::new();
    // Put template values
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("highlight_color", HIGHLIGHT_COLOR);
    context.insert("color", COLOR);
    context.insert("rounding", &ROUNDING.min(PADDING));
    context.insert("fill_width", &fill_width);
    context.insert("fill_height", &fill_height);
    context.insert("highlight_width", &HIGHLIGHT_WIDTH);
    context.insert("fin_width", &SHARK_FIN_WIDTH);
    context.insert("fin_height", &SHARK_FIN_HEIGHT);
    context.insert("fin_rounding", &SHARK_FIN_ROUNDING);
    context.insert("opacity", &OPACITY);

    let svg = tera.render("bubble", &context).unwrap();
    ChatBubble {
        svg,
        vb_width: width,
        vb_height: height,
    }
}

#[cfg(test)]
#[test]
pub fn test_is_svg() {
    use usvg::{Options, TreeParsing};
    let ChatBubble { svg, .. } = create(100.0, 50.0);
    assert!(usvg::Tree::from_str(&svg, &Options::default()).is_ok());
}

#[test]
pub fn test_vb_size() {
    use usvg::{Options, TreeParsing};
    let ChatBubble {
        svg,
        vb_width: vbox_width,
        vb_height,
    } = create(100.0, 50.0);
    assert!(usvg::Tree::from_str(&svg, &Options::default()).is_ok());
}
