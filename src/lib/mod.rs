use std::collections::HashSet;

use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;

pub mod entities;
pub mod text;

#[derive(Debug, Copy, Clone)]
pub struct Controls {
  pub up: Scancode,
  pub down: Scancode,
}

pub struct GameState {
  pub view_port: (u32, u32),
  pub keyboard_state: HashSet<Scancode>,
  pub playing: bool,
  pub player_rects: Vec<Rect>,
  pub player_scores: (usize, usize),
}
