use tera::{Context, Tera};

const BUBBLE_SVG: &str = include_str!("bubble.svg.j2");

/// Top color of the text bubble gradient
const COLOR1: &str = "#32c7f5";

/// Bottom color of the text bubble gradient
const COLOR2: &str = "#86e2ff";

/// Highlight color of the text bubble
const STROKE_COLOR: &str = "#86e2ff";

/// Opacity of the chat bubble
const OPACITY: &str = "80%";

/// How wide the shark fin is
const SHARK_FIN_WIDTH: f32 = 30.0; // px

/// How tall the shark fin is
const SHARK_FIN_HEIGHT: f32 = 12.5; // px

/// How thick the highlight is
const STROKE_WIDTH: f32 = 10.0; // px

// Empty space on all sides of the text
const PADDING: f32 = 15.0; // px

// Rounding needs to be smaller than padding
const ROUNDING: f32 = 15.0; // px

pub struct ChatBubble {
    /// The SVG contents
    pub svg: String,
}

/// Create a chat bubble
pub fn create(inner_width: f32, inner_height: f32) -> ChatBubble {
    let inner_width = inner_width.max(SHARK_FIN_WIDTH);
    let inner_height = inner_height.max(PADDING.max(ROUNDING));

    // Normal viewbox size
    let width = PADDING.max(ROUNDING) * 2.0 + inner_width;
    let height = PADDING.max(ROUNDING) * 2.0 + inner_height;

    let (vb_x1, vb_x2) = (-STROKE_WIDTH / 2.0, width + STROKE_WIDTH);
    let (vb_y1, vb_y2) = (
        -STROKE_WIDTH / 2.0,
        height + SHARK_FIN_HEIGHT + STROKE_WIDTH / 2.0,
    );

    let fill_width = inner_width + PADDING.max(ROUNDING) * 2.0 - PADDING.min(ROUNDING) * 2.0;
    let fill_height = inner_height + PADDING.max(ROUNDING) * 2.0 - PADDING.min(ROUNDING) * 2.0;

    // Create template
    let mut tera = Tera::default();
    tera.add_raw_template("bubble", BUBBLE_SVG).unwrap();

    let mut context = Context::new();
    // Put template values
    context.insert("width", &width);
    context.insert("vb_x1", &vb_x1);
    context.insert("vb_y1", &vb_y1);
    context.insert("vb_x2", &vb_x2);
    context.insert("vb_y2", &vb_y2);
    context.insert("highlight_color", STROKE_COLOR);
    context.insert("color1", COLOR1);
    context.insert("color2", COLOR2);
    context.insert("rounding", &ROUNDING.min(PADDING));
    context.insert("fill_width", &fill_width);
    context.insert("fill_height", &fill_height);
    context.insert("highlight_width", &STROKE_WIDTH);
    context.insert("fin_width", &SHARK_FIN_WIDTH);
    context.insert("fin_height", &SHARK_FIN_HEIGHT);
    context.insert("opacity", &OPACITY);

    let svg = tera.render("bubble", &context).unwrap();
    ChatBubble { svg }
}

#[cfg(test)]
mod tests {
    use crate::{create, ChatBubble};

    #[test]
    pub fn test_is_svg() {
        let ChatBubble { svg, .. } = create(100.0, 50.0);
        assert!(usvg::Tree::from_str(&svg, &usvg::Options::default(), &Default::default()).is_ok());
    }
    #[test]
    pub fn test_vb_size() {
        let ChatBubble { svg, .. } = create(100.0, 50.0);
        assert!(usvg::Tree::from_str(&svg, &usvg::Options::default(), &Default::default()).is_ok());
    }
}
