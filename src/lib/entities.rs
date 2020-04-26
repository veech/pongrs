use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use super::shapes;

const PLAYER_HEIGHT: u32 = 150;
const PLAYER_WIDTH: u32 = 20;
const PLAYER_COLOR: Color = Color::RGB(255, 255, 255);

pub trait Entity {
  fn render(&self, canvas: &mut Canvas<Window>);
}

pub struct Player<'a> {
  square: shapes::Square<'a>,
}

impl<'a> Player<'a> {
  pub fn new(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
  ) -> Player<'a> {
    let mut square = shapes::Square::new(PLAYER_HEIGHT, PLAYER_WIDTH);
    square.set_color(canvas, texture_creator, PLAYER_COLOR);

    Player { square }
  }

  pub fn set_position(&mut self, x: i32, y: i32) {
    self.square.position = (x, y);
  }
}

impl Entity for Player<'_> {
  fn render(&self, canvas: &mut Canvas<Window>) {
    self.square.draw_to_canvas(canvas);
  }
}
