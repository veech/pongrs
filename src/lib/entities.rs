use std::collections::HashSet;

use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use super::shapes;
use super::Size;

const PLAYER_HEIGHT: u32 = 150;
const PLAYER_WIDTH: u32 = 20;
const PLAYER_COLOR: Color = Color::RGB(255, 255, 255);
const PLAYER_VELOCITY: i32 = 25;

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

pub trait Entity {
  fn update(&mut self, game_state: &GameState);
  fn render(&self, canvas: &mut Canvas<Window>);
}

pub struct Player<'a> {
  controls: Controls,

  square: shapes::Square<'a>,
}

impl<'a> Player<'a> {
  pub fn new(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    controls: Controls,
  ) -> Player<'a> {
    let mut square = shapes::Square::new(PLAYER_HEIGHT, PLAYER_WIDTH);
    square.set_color(canvas, texture_creator, PLAYER_COLOR);

    Player { controls, square }
  }

  pub fn position(&self) -> (i32, i32) {
    self.square.position
  }

  pub fn size(&self) -> Size {
    Size {
      height: self.square.height,
      width: self.square.width,
    }
  }

  pub fn set_position(&mut self, x: i32, y: i32) {
    self.square.position = (x, y);
  }

  pub fn move_by(&mut self, dx: i32, dy: i32) {
    let (x, y) = self.square.position;
    self.square.position = (x + dx, y + dy);
  }
}

impl Entity for Player<'_> {
  fn update(&mut self, game_state: &GameState) {
    let view_port = &game_state.view_port;
    let (x, y) = self.position();

    if game_state.keyboard_state.contains(&self.controls.up) {
      if (y - PLAYER_VELOCITY) > 0 {
        self.move_by(0, -25);
      } else {
        self.set_position(x, 0);
      }
    }

    if game_state.keyboard_state.contains(&self.controls.down) {
      if (y + PLAYER_VELOCITY) < (view_port.height - self.square.height) as i32 {
        self.move_by(0, 25);
      } else {
        self.set_position(x, (view_port.height - self.square.height) as i32);
      }
    }
  }

  fn render(&self, canvas: &mut Canvas<Window>) {
    self.square.draw_to_canvas(canvas);
  }
}
