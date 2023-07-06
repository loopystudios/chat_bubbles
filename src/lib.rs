use tera::{Context, Tera};

const SVG: &str = include_str!("bubble.svg");

/// Color of the text bubble
const COLOR: &str = "#32c7f5";

/// Highlight color of the text bubble
const HIGHLIGHT_COLOR: &str = "#86e2ff";

/// How wide the shark fin is
const SHARK_FIN_WIDTH: f32 = 35.0; // px

/// How tall the shark fin is
const SHARK_FIN_HEIGHT: f32 = 30.0; // px

const HIGHLIGHT_WIDTH: f32 = 20.0; // px

// Empty space on all sides of the text
const PADDING: f32 = 50.0; // px

// Rounding needs to be smaller than padding
const ROUNDING: f32 = 30.0; // px

pub fn create(inner_width: f32, inner_height: f32) -> (String, f32, f32) {
    let width = PADDING.max(ROUNDING) * 2.0 + inner_width;
    let height = PADDING.max(ROUNDING) * 2.0 + inner_height;

    let fill_width = inner_width + PADDING.max(ROUNDING) * 2.0 - PADDING.min(ROUNDING) * 2.0;
    let fill_height = inner_height + PADDING.max(ROUNDING) * 2.0 - PADDING.min(ROUNDING) * 2.0;

    // Create template
    let mut tera = Tera::default();
    tera.add_raw_template("bubble.svg", SVG).unwrap();

    let mut context = Context::new();
    // Put template values
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("highlight_color", HIGHLIGHT_COLOR);
    context.insert("color", COLOR);
    context.insert("start_x", &ROUNDING);
    context.insert("start_y", &0);
    context.insert("rounding", &ROUNDING.min(PADDING));
    context.insert("fill_width", &fill_width);
    context.insert("fill_height", &fill_height);
    context.insert("highlight_width", &HIGHLIGHT_WIDTH);
    context.insert("fin_width", &SHARK_FIN_WIDTH);
    context.insert("fin_height", &SHARK_FIN_HEIGHT);

    (tera.render("bubble.svg", &context).unwrap(), width, height)
}

#[cfg(test)]
#[test]
pub fn test_is_svg() {
    use usvg::{Options, TreeParsing};
    let svg = create(100.0, 50.0);
    assert!(usvg::Tree::from_str(&svg, &Options::default()).is_ok());
}
