use bevy::{prelude::Vec2, render::color::Color};

pub const INITIAL_CENTIPEDE_LENGTH: usize = 30;

pub const DEFAULT_SPEED: f32 = 100.0;

pub const HEAD_SIZE: f32 = 8.0;
pub const HEAD_COLOR: Color = Color::BLUE;

pub const MARKER_SIZE: f32 = 6.0;
pub const MARKER_COLOR: Color = Color::BLUE;

pub const TAIL_SIZE: Vec2 = Vec2::new(24.0, 6.0);
pub const TAIL_COLOR: Color = Color::BLUE;
pub const PURGED_COLOR: Color = Color::RED;
pub const TAIL_DISTANCE: f32 = 30.0;

pub const GATE_MIN_WIDTH: f32 = 100.0;
pub const GATE_MAX_WIDTH: f32 = 180.0;
pub const POLL_COLOR: Color = Color::ORANGE_RED;
pub const POLL_SIZE: f32 = 8.0;
pub const BAR_COLOR: Color = Color::LIME_GREEN;
pub const BAR_DIAMETER: f32 = 10.0;
pub const GATE_SPAWN_PER_SECONDS: f64 = 2.0;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";

pub const FPS_PREFIX: &str = "FPS:";
pub const FPS_SIZE: f32 = 24.0;
pub const FPS_COLOR: Color = Color::WHITE;

pub const SCORE_PREFIX: &str = "SCORE:";
pub const SPEED_PREFIX: &str = "SPEED:";
pub const SPEED_UP: f32 = 3.0;
pub const HIGH_SCORE_PREFIX: &str = "HIGH:";
pub const TAIL_PREFIX: &str = "TAIL:";
pub const SCORE_SIZE: f32 = 24.0;
pub const SCORE_COLOR: Color = Color::WHITE;
pub const GATE_NOT_SPAWN_DISTANCE_TO_HEAD: f32 = 100.0;

pub const PANEL_Z: f32 = 10.0;
pub const GATE_Z: f32 = 20.0;
pub const MARKER_Z: f32 = 30.0;
pub const CENTIPEDE_Z: f32 = 40.0;
pub const PANEL_SIZE: f32 = 50.0;
pub const PANEL_X_COUNT: isize = 25;
pub const PANEL_Y_COUNT: isize = 13;
pub const PANEL_COLOR: Color = Color::GRAY;
pub const BOARD_Y_SIZE: f32 = PANEL_Y_COUNT as f32 * PANEL_SIZE;
pub const BOARD_Y_BORDER: f32 = BOARD_Y_SIZE / 2.0;
pub const BOARD_X_SIZE: f32 = PANEL_X_COUNT as f32 * PANEL_SIZE;
pub const BOARD_X_BORDER: f32 = BOARD_X_SIZE / 2.0;
pub const BOARD_MARGIN: f32 = 25.0;
