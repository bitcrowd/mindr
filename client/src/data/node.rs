use super::DEFAULT_COLOR;
use std::sync::OnceLock;
use uuid::Uuid;

use fontdue::Font;
static FONT_BYTES: &[u8] = include_bytes!("../../assets/fonts/Roboto-Light.ttf");
static FONT: OnceLock<Font> = OnceLock::new();
fn get_font() -> &'static Font {
    FONT.get_or_init(|| {
        Font::from_bytes(FONT_BYTES, fontdue::FontSettings::default()).expect("Failed to load font")
    })
}
const FONT_SIZE: f32 = 14.0;

pub fn measure_text_width(text: &str) -> f32 {
    text.chars()
        .map(|ch| {
            let (metrics, _) = get_font().rasterize(ch, FONT_SIZE);
            metrics.advance_width
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RelativeLocation {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RenderedNode {
    pub id: Uuid,
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub parent_id: Option<Uuid>,
    pub color: Option<String>,
    pub rendered_color: String,
    pub estimate: Option<f64>,
    pub estimate_rollup: f64,
    pub progress: i64,
}

const TEXT_PADDING: f32 = 10.0;
impl RenderedNode {
    pub fn new(
        id: Uuid,
        (x, y): (f32, f32),
        parent_id: Option<Uuid>,
        text: String,
        color: Option<String>,
        estimate: Option<f64>,
        progress: i64,
    ) -> Self {
        Self {
            id,
            x,
            y,
            text,
            parent_id,
            color,
            estimate,
            progress,
            estimate_rollup: 0.0,
            rendered_color: DEFAULT_COLOR.to_string(),
        }
    }

    pub fn width(&self) -> f32 {
        (self
            .text
            .lines()
            .fold(0f32, |acc, line| acc.max(measure_text_width(line))))
        .max(80.0f32)
            + TEXT_PADDING * 2.0
    }

    pub fn height(&self) -> f32 {
        let lines = if self.text.ends_with("\n") || self.text.ends_with("\n\r") {
            self.text.lines().count() + 1
        } else {
            self.text.lines().count()
        }
        .max(1);
        FONT_SIZE * lines as f32 + FONT_SIZE * 0.2 * (lines as f32 - 1.0) + TEXT_PADDING * 2.0
    }

    pub fn font_size(&self) -> f32 {
        FONT_SIZE
    }

    pub fn on(&self, (x, y): (f32, f32)) -> Option<RelativeLocation> {
        use RelativeLocation::*;
        let (w, h) = (self.width(), self.height());
        let (dx, dy) = (x - self.x, y - self.y);
        let (hw, hh) = (w / 2.0, h / 2.0);

        // Tolerance: 30% of smaller dimension, clamped for sanity
        let tol = (w.min(h) * 0.30).clamp(6.0, 16.0);

        if dx.abs() > hw + tol || dy.abs() > hh + tol {
            return None;
        }

        match (dx, dy) {
            (_, d) if (d + hh).abs() <= tol => Some(Top),
            (_, d) if (d - hh).abs() <= tol => Some(Bottom),
            (d, _) if (d + hw).abs() <= tol => Some(Left),
            (d, _) if (d - hw).abs() <= tol => Some(Right),
            _ if dx.abs() <= hw && dy.abs() <= hh => Some(Center),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn make_node(text: &str) -> RenderedNode {
        RenderedNode::new(
            Uuid::new_v4(),
            (100.0, 100.0),
            None,
            text.to_string(),
            None,
            None,
            0,
        )
    }

    // -------------------------
    // measure_text_width
    // -------------------------
    #[test]
    fn test_measure_text_width_nonzero() {
        let w = measure_text_width("Hello");
        assert!(w > 0.0);
    }

    #[test]
    fn test_measure_text_width_adds_more_for_longer_text() {
        let w1 = measure_text_width("Hi");
        let w2 = measure_text_width("Hello, world!");
        assert!(w2 > w1);
    }

    // -------------------------
    // width()
    // -------------------------
    #[test]
    fn test_width_has_minimum() {
        let node = make_node("Hi");
        let w = node.width();

        assert!(w >= 80.0, "Width should enforce minimum");
        assert!(w > 80.0, "Width should include padding");
    }

    #[test]
    fn test_width_multiline_takes_longest_line() {
        let node = make_node("short\nthis is a much longer line\nmid");
        let width = node.width();

        let width_short = measure_text_width("short") + 20.0;
        let width_long = measure_text_width("this is a much longer line") + 20.0;

        assert!(width >= width_long);
        assert!(width > width_short);
    }

    // -------------------------
    // height()
    // -------------------------
    #[test]
    fn test_height_single_line() {
        let node = make_node("Hello");
        let h = node.height();

        let expected_min = FONT_SIZE + TEXT_PADDING * 2.0;
        assert!(h >= expected_min);
    }

    #[test]
    fn test_height_multiple_lines() {
        let node1 = make_node("One line");
        let node2 = make_node("Line 1\nLine 2");

        assert!(node2.height() > node1.height());
    }

    #[test]
    fn test_height_handles_trailing_newline() {
        let node1 = make_node("Line 1\nLine 2");
        let node2 = make_node("Line 1\nLine 2\n"); // Should count as 3 lines

        assert!(node2.height() > node1.height());
    }

    // -------------------------
    // on()
    // -------------------------
    #[test]
    fn test_on_center_detection() {
        let node = make_node("Center test");
        let loc = node.on((100.0, 100.0));

        assert_eq!(loc, Some(RelativeLocation::Center));
    }

    #[test]
    fn test_on_outside_returns_none() {
        let node = make_node("Test");
        let loc = node.on((500.0, 500.0));

        assert_eq!(loc, None);
    }

    #[test]
    fn test_on_top_detection() {
        let node = make_node("Test");
        let h = node.height();
        let loc = node.on((100.0, 100.0 - h / 2.0));

        assert_eq!(loc, Some(RelativeLocation::Top));
    }

    #[test]
    fn test_on_bottom_detection() {
        let node = make_node("Test");
        let h = node.height();
        let loc = node.on((100.0, 100.0 + h / 2.0));

        assert_eq!(loc, Some(RelativeLocation::Bottom));
    }

    #[test]
    fn test_on_left_detection() {
        let node = make_node("Test");
        let w = node.width();
        let loc = node.on((100.0 - w / 2.0, 100.0));

        assert_eq!(loc, Some(RelativeLocation::Left));
    }

    #[test]
    fn test_on_right_detection() {
        let node = make_node("Test");
        let w = node.width();
        let loc = node.on((100.0 + w / 2.0, 100.0));

        assert_eq!(loc, Some(RelativeLocation::Right));
    }

    #[test]
    fn test_on_near_edge_still_detected_with_tolerance() {
        let node = make_node("Test");
        let w = node.width();

        // Slightly outside exact edge, but within tolerance
        let loc = node.on((100.0 + w / 2.0 + 8.0, 100.0));

        assert_eq!(loc, Some(RelativeLocation::Right));
    }
}
