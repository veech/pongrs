use sdl2::keyboard::Scancode;
use std::collections::HashSet;

pub mod entities;
pub mod shapes;

#[derive(Debug, Copy, Clone)]
pub struct Size {
  pub height: u32,
  pub width: u32,
}

#[derive(Debug)]
pub struct GameState {
  // TODO: use a more descriptive type than a touple
  pub view_port: Size,
  pub keyboard_state: HashSet<Scancode>,
}

pub struct Controls {
  pub up: Scancode,
  pub down: Scancode,
}
