use sdl2::keyboard::Scancode;
use std::collections::HashSet;

pub mod entities;
pub mod shapes;

#[derive(Debug, Copy, Clone)]
pub struct Size {
  pub height: u32,
  pub width: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Controls {
  pub up: Scancode,
  pub down: Scancode,
}

#[derive(Debug)]
pub struct GameState {
  // TODO: use a more descriptive type than a touple
  pub view_port: Size,
  pub keyboard_state: HashSet<Scancode>,
}
