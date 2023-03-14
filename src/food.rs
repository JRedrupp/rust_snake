use speedy2d::color::Color;
use speedy2d::shape::Rectangle;
use speedy2d::Graphics2D;

use crate::scoreboard::Scoreboard;
use crate::settings::{CELL_HEIGHT, CELL_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Food {
    x: i32,
    y: i32,
}

impl Food {
    pub fn new() -> Self {
        // Create a bit of food in a random location
        Food {
            x: (rand::random::<u32>() % (WINDOW_WIDTH / CELL_WIDTH)) as i32 * CELL_WIDTH as i32,
            y: ((rand::random::<u32>() % ((WINDOW_HEIGHT - Scoreboard::BOARD_HEIGHT) / CELL_HEIGHT))
                as i32
                * CELL_HEIGHT as i32)
                + Scoreboard::BOARD_HEIGHT as i32,
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_rectangle(
            Rectangle::from_tuples(
                (self.x as f32, self.y as f32),
                (
                    (self.x + CELL_WIDTH as i32) as f32,
                    (self.y + CELL_HEIGHT as i32) as f32,
                ),
            ),
            Color::RED,
        );
    }

    pub fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
