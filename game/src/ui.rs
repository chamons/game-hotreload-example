use crate::{
    example::host::types::{GameColor, Position, Size},
    WHITE,
};

#[cfg(not(feature = "hotreload"))]
use crate::direct::GameScreen;
#[cfg(feature = "hotreload")]
use crate::hotreload::GameScreen;

impl From<(f32, f32)> for Position {
    fn from(value: (f32, f32)) -> Self {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(f32, f32)> for Size {
    fn from(value: (f32, f32)) -> Self {
        Size {
            width: value.0,
            height: value.1,
        }
    }
}

pub enum TextSize {
    Title,
    Standard,
}

impl Into<f32> for TextSize {
    fn into(self) -> f32 {
        match self {
            TextSize::Title => 40.0,
            TextSize::Standard => 20.0,
        }
    }
}

pub trait ScreenExt {
    fn text(&self, text: &str, position: (f32, f32), size: TextSize, color: GameColor);

    fn standard_text(&self, text: &str, position: (f32, f32)) {
        self.text(text, position, TextSize::Standard, WHITE);
    }
}

impl ScreenExt for GameScreen {
    fn text(&self, text: &str, position: (f32, f32), size: TextSize, color: GameColor) {
        self.draw_text(text, (position.0, position.1).into(), size.into(), color);
    }
}