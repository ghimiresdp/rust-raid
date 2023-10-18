mod util;

use std::time::{Duration, Instant};

use eframe::{
    egui::{self, Ui},
    epaint::{Color32, Pos2, Rect, RectShape, Rounding, Vec2},
};
use rand::{random, Rng};

use util::generate_random_position;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Bottom,
    Left,
    Right,
    Top,
}

pub(crate) struct Snake {
    position: Pos2,
    body: Vec<Direction>, // each part of a body should
}
impl Snake {
    fn new() -> Self {
        Self {
            position: Pos2::new(270.0, 310.0),
            body: vec![
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Top,
                Direction::Top,
                Direction::Top,
                Direction::Top,
                Direction::Right,
                Direction::Right,
                Direction::Right,
            ],
        }
    }
    pub(crate) fn make_move(&mut self, direction: Direction) {
        let head = self.body[0];
        let step = 25.0;
        match direction {
            Direction::Bottom => {
                if head != Direction::Top {
                    self.position.y += step;
                }
            }
            Direction::Left => {
                if head != Direction::Right {
                    self.position.x -= step;
                }
            }
            Direction::Right => {
                if head != Direction::Left {
                    self.position.x += step;
                }
            }
            Direction::Top => {
                if head != Direction::Bottom {
                    self.position.y -= step;
                }
            }
        };
        self.body.insert(0, direction);
        self.body.pop();
    }

    pub(crate) fn auto_move(&mut self) {
        // move to the position of head
        self.make_move(self.body[0])
    }

    pub(crate) fn render(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Create a game arena
            let (mut x, mut y) = (self.position.x, self.position.y);
            for item in &self.body {
                ui.painter().add(egui::Shape::Rect(RectShape::filled(
                    Rect::from_center_size(Pos2::new(x%500.0, y%500.0), Vec2::new(25.0, 25.0)),
                    Rounding::same(8.0),
                    Color32::BLUE,
                )));
                match item {
                    Direction::Bottom => y -= 25.0,
                    Direction::Left => x += 25.0,
                    Direction::Right => x -= 25.0,
                    Direction::Top => y += 25.0,
                }
            }
        });
    }
}

#[derive(Clone, Copy)]
pub struct Food {
    is_edible: bool,
    position: (u32, u32),
    weight: u32,
}

impl Food {
    fn new() -> Self {
        Self {
            is_edible: true,
            position: generate_random_position(),
            weight: 1,
        }
    }
}

pub struct Game {
    level: u64, // speed control
    snake: Snake,
    food: Food,
    last_update: Instant,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            level: 1,
            snake: Snake::new(),
            food: Food::new(),
            last_update: Instant::now(),
        }
    }
    pub fn get_food(&self) -> Food {
        self.food
    }
    pub(crate) fn set_level(&mut self, level: u64) {
        if self.level != level {
            self.level = level;
        }
    }

    pub(crate) fn render(&mut self, ui: &mut Ui) {
        ui.painter().add(egui::Shape::Rect(RectShape::filled(
            Rect::from_min_size(Pos2::new(20.0, 80.0), Vec2::new(500.0, 500.0)),
            Rounding::same(10.0),
            Color32::DARK_GRAY,
        )));
        if self.last_update.elapsed() >= Duration::from_millis(1000 / self.level) {
            self.last_update = Instant::now();
            self.snake.auto_move();
        }
        self.snake.render(ui);
    }
}
