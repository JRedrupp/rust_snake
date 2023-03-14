use chrono::Utc;
use game::Game;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

mod food;
mod game;
mod scoreboard;
mod snake;

const APP_NAME: &str = "Rust Snake";
const WINDOW_HEIGHT: u32 = 704;
const WINDOW_WIDTH: u32 = 640;
const CELL_HEIGHT: u32 = 20;
const CELL_WIDTH: u32 = 20;
const FRAME_RATE: f32 = 10.0;
const CREATE_FOOD_CHANCE: f32 = 0.01;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct MyWindowHandler {
    game: Game,
    frame_rate: f32,
}

impl MyWindowHandler {
    fn new() -> Self {
        MyWindowHandler {
            game: Game::new(),
            frame_rate: FRAME_RATE,
        }
    }

    fn change_speed(&mut self, speed: f32) {
        self.frame_rate += speed;
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        // Get the current time
        let now = Utc::now();

        graphics.clear_screen(Color::BLACK);

        // Update the game
        if !self.game.is_over() {
            self.game.update();
        }

        // Draw the game
        self.game.draw(graphics);

        // Get the End time
        let end = Utc::now();

        // Calculate the elapsed time
        let elapsed = end - now;

        // Calculate the time to sleep
        let sleep_time = (1.0 / self.frame_rate) - elapsed.num_milliseconds() as f32 / 1000.0;

        // Sleep for the remaining time
        std::thread::sleep(std::time::Duration::from_millis(
            (sleep_time * 1000.0) as u64,
        ));

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        _: &mut WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        _: speedy2d::window::KeyScancode,
    ) {
        match virtual_key_code {
            Some(speedy2d::window::VirtualKeyCode::Up) => {
                self.game.change_direction(Direction::Up);
            }
            Some(speedy2d::window::VirtualKeyCode::Down) => {
                self.game.change_direction(Direction::Down);
            }
            Some(speedy2d::window::VirtualKeyCode::Left) => {
                self.game.change_direction(Direction::Left);
            }
            Some(speedy2d::window::VirtualKeyCode::Right) => {
                self.game.change_direction(Direction::Right);
            }
            Some(speedy2d::window::VirtualKeyCode::Escape) => {
                std::process::exit(0);
            }
            Some(speedy2d::window::VirtualKeyCode::Comma) => {
                self.change_speed(-1.0);
            }
            Some(speedy2d::window::VirtualKeyCode::Period) => {
                self.change_speed(1.0);
            }
            Some(speedy2d::window::VirtualKeyCode::Space) => {
                self.game.pause();
            }
            Some(speedy2d::window::VirtualKeyCode::R) => self.game.restart(),
            _ => {}
        }
    }
}

fn main() {
    let window = Window::new_centered(APP_NAME, (WINDOW_WIDTH, WINDOW_HEIGHT)).unwrap();
    window.run_loop(MyWindowHandler::new())
}
