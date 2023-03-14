use rand;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::font::{Font, TextAlignment, TextLayout, TextOptions};
use speedy2d::Graphics2D;

use crate::food::Food;
use crate::scoreboard::Scoreboard;
use crate::snake::Snake;
use crate::{Direction, CREATE_FOOD_CHANCE, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Game {
    snake: Snake,
    foods: Vec<Food>,
    scoreboard: Scoreboard,
    font: Font,
    is_paused: bool,
}

impl Game {
    pub fn new() -> Self {
        let font = Font::new(include_bytes!("../assets/fonts/NotoSans-Regular.ttf")).unwrap();
        Game {
            snake: Snake::new(),
            foods: vec![Food::new()],
            font: font,
            scoreboard: Scoreboard::new(),
            is_paused: false,
        }
    }

    pub fn update(&mut self) {
        if self.is_paused {
            return;
        }

        // Check if the snake has eaten any food
        let mut eaten_food = false;
        let mut eaten_food_index = 0;
        for (index, food) in self.foods.iter().enumerate() {
            if self.snake.intersects_point(food.get_pos()) {
                eaten_food = true;
                eaten_food_index = index;
                break;
            }
        }

        // If the snake has eaten food, remove the food and add a new one
        if eaten_food {
            self.foods.remove(eaten_food_index);
            self.scoreboard.increase_score();
        }
        self.snake.update(eaten_food);

        // Create a new food with a random chance
        if rand::random::<f32>() < CREATE_FOOD_CHANCE || self.foods.len() == 0 {
            self.foods.push(Food::new());
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        self.scoreboard.draw(graphics, &self.font);

        // If Paused, draw the pause text
        if self.is_paused {
            let text_layout = self.font.layout_text(
                "Paused \n Press P to unpause",
                48.0,
                TextOptions::new()
                    .with_wrap_to_width((WINDOW_WIDTH / 2) as f32, TextAlignment::Center),
            );
            let position = Vec2::new((WINDOW_WIDTH / 4) as f32, (WINDOW_HEIGHT / 2) as f32 - 24.0);
            graphics.draw_text(position, Color::WHITE, &text_layout);
            return;
        }

        self.snake.draw(graphics);
        for food in &self.foods {
            food.draw(graphics);
        }

        // Draw the game over text if the game is over
        if self.snake.is_over() {
            let text_layout = self.font.layout_text(
                "Game Over \n Press R to restart",
                48.0,
                TextOptions::new()
                    .with_wrap_to_width((WINDOW_WIDTH / 2) as f32, TextAlignment::Center),
            );
            let position = Vec2::new((WINDOW_WIDTH / 4) as f32, (WINDOW_HEIGHT / 2) as f32 - 24.0);
            graphics.draw_text(position, Color::WHITE, &text_layout);
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    pub fn is_over(&self) -> bool {
        self.snake.is_over()
    }

    pub fn restart(&mut self) {
        if self.is_paused {
            return;
        }
        self.snake = Snake::new();
        self.foods = vec![Food::new()];
        self.scoreboard.new_game()
    }

    pub fn pause(&mut self) {
        if self.is_paused {
            self.is_paused = false;
        } else {
            self.is_paused = true;
        }
    }
}
