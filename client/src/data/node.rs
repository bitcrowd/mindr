use uuid::Uuid;

use super::DEFAULT_COLOR;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RelativeLocation {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

const FONT_SIZE: f32 = 14.0;
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
            rendered_color: DEFAULT_COLOR.to_string(),
        }
    }

    pub fn width(&self) -> f32 {
        (self.text.lines().fold(0, |acc, line| acc.max(line.len())) as f32 * FONT_SIZE * 0.6)
            .max(80.0)
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
