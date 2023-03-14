use rand;
use speedy2d::color::Color;
use speedy2d::shape::Rectangle;
use speedy2d::Graphics2D;

use crate::scoreboard::Scoreboard;
use crate::settings::{CELL_HEIGHT, CELL_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::Direction;

pub struct SnakePoint {
    x: i32,
    y: i32,
}

impl SnakePoint {
    fn draw(&self, graphics: &mut Graphics2D) {
        let x = self.x as f32;
        let y = self.y as f32;
        graphics.draw_rectangle(
            Rectangle::from_tuples((x, y), (x + CELL_WIDTH as f32, y + CELL_HEIGHT as f32)),
            Color::WHITE,
        );
    }
}

pub struct Snake {
    pub points: Vec<SnakePoint>,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            points: vec![SnakePoint {
                x: (rand::random::<u32>() % (WINDOW_WIDTH / CELL_WIDTH)) as i32 * CELL_WIDTH as i32,
                y: ((rand::random::<u32>()
                    % ((WINDOW_HEIGHT - Scoreboard::BOARD_HEIGHT) / CELL_HEIGHT))
                    as i32
                    * CELL_HEIGHT as i32)
                    + Scoreboard::BOARD_HEIGHT as i32,
            }],
            direction: Direction::Right,
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        for point in &self.points {
            point.draw(graphics);
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        // Ensure it's not the opposite direction
        match direction {
            Direction::Up => {
                if self.direction == Direction::Down {
                    return;
                }
            }
            Direction::Down => {
                if self.direction == Direction::Up {
                    return;
                }
            }
            Direction::Left => {
                if self.direction == Direction::Right {
                    return;
                }
            }
            Direction::Right => {
                if self.direction == Direction::Left {
                    return;
                }
            }
        }
        self.direction = direction;
    }

    fn move_snake(&mut self, eaten_food: bool) {
        // Get the last point
        let first_point = self.points.first().unwrap();

        // Create a new point
        let mut new_point = SnakePoint {
            x: first_point.x,
            y: first_point.y,
        };

        // Move the new point
        match self.direction {
            Direction::Up => new_point.y -= CELL_HEIGHT as i32,
            Direction::Down => new_point.y += CELL_HEIGHT as i32,
            Direction::Left => new_point.x -= CELL_WIDTH as i32,
            Direction::Right => new_point.x += CELL_WIDTH as i32,
        }

        // Wrap around the screen
        if new_point.x >= WINDOW_WIDTH as i32 {
            new_point.x = 0;
        } else if new_point.x < 0 {
            new_point.x = WINDOW_WIDTH as i32;
        }

        if new_point.y >= (WINDOW_HEIGHT) as i32 {
            new_point.y = Scoreboard::BOARD_HEIGHT as i32;
        } else if new_point.y < Scoreboard::BOARD_HEIGHT as i32 {
            new_point.y = WINDOW_HEIGHT as i32;
        }

        // Add the new point to the front of the vector
        self.points.insert(0, new_point);

        // Remove the last point
        if !eaten_food {
            self.points.pop();
        }
    }

    pub fn update(&mut self, eaten_food: bool) {
        self.move_snake(eaten_food);
    }

    pub fn is_over(&self) -> bool {
        // Get the first point
        let first_point = self.points.first().unwrap();

        // Check if the snake has eaten itself
        for point in self.points.iter().skip(1) {
            if point.x == first_point.x && point.y == first_point.y {
                return true;
            }
        }

        false
    }

    pub fn intersects_point(&self, xy: (i32, i32)) -> bool {
        // Get the first point
        let first_point = self.points.first().unwrap();

        if (first_point.x - xy.0).abs() < 2 && (first_point.y - xy.1).abs() < 2 {
            return true;
        }
        return false;
    }
}
