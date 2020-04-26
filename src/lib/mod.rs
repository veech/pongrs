use sdl2::keyboard::Scancode;
use std::collections::HashSet;

pub mod entities;

#[derive(Debug, Copy, Clone)]
pub struct Controls {
  pub up: Scancode,
  pub down: Scancode,
}

pub struct GameState {
  pub view_port: (u32, u32),
  pub keyboard_state: HashSet<Scancode>,
}
