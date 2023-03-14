use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::font::{Font, TextAlignment, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::Graphics2D;

use crate::WINDOW_WIDTH;

pub struct Scoreboard {
    score: u32,
    high_score: u32,
}

impl Scoreboard {
    pub const BOARD_HEIGHT: u32 = 64;
    pub fn new() -> Self {
        Scoreboard {
            score: 0,
            high_score: 0,
        }
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D, font: &Font) {
        // Draw the scoreboard background
        graphics.draw_rectangle(
            Rectangle::from_tuples(
                (0.0, 0.0),
                (WINDOW_WIDTH as f32, Scoreboard::BOARD_HEIGHT as f32),
            ),
            Color::GRAY,
        );

        let text_layout_score = font.layout_text(
            format!("Score: {}", self.score).as_str(),
            48.0,
            TextOptions::new().with_wrap_to_width(500.0, TextAlignment::Left),
        );
        let position = Vec2::new(10.0, 10.0);
        graphics.draw_text(position, Color::WHITE, &text_layout_score);

        let text_layout_high_score = font.layout_text(
            format!("High Score: {}", self.high_score).as_str(),
            48.0,
            TextOptions::new().with_wrap_to_width(500.0, TextAlignment::Left),
        );
        let position = Vec2::new((WINDOW_WIDTH / 2) as f32, 10.0);
        graphics.draw_text(position, Color::WHITE, &text_layout_high_score);
    }

    pub fn new_game(&mut self) {
        self.score = 0;
    }
}
