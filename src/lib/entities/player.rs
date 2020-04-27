use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::super::{Controls, GameState};
use super::Entity;

const PLAYER_COLOR: Color = Color::RGB(255, 255, 255);
const PLAYER_VELOCITY: i32 = 25;
const PLAYER_WIDTH: u32 = 20;
const PLAYER_HEIGHT: u32 = 150;

pub struct Player {
  size: (u32, u32),
  color: Color,
  position: Point,

  controls: Controls,
}

impl Player {
  pub fn new(controls: Controls) -> Player {
    Player {
      color: PLAYER_COLOR,
      position: Point::new(0, 0),
      size: (PLAYER_WIDTH, PLAYER_HEIGHT),

      controls,
    }
  }

  pub fn set_position(&mut self, pos: (i32, i32)) {
    let (x, y) = pos;
    self.position = Point::new(x, y);
  }

  pub fn set_center(&mut self, pos: (i32, i32)) {
    let (x, y) = pos;
    let (width, height) = self.size;

    self.position = Point::new(x - (width / 2) as i32, y - (height / 2) as i32);
  }

  pub fn move_by(&mut self, delta: (i32, i32)) {
    let (dx, dy) = delta;
    let pos = self.position;

    self.position = Point::new(pos.x + dx, pos.y + dy);
  }

  pub fn as_rect(&self) -> Rect {
    let pos = self.position;
    let (width, height) = self.size;

    Rect::new(pos.x(), pos.y(), width, height)
  }
}

impl Entity for Player {
  fn update(&mut self, game_state: &mut GameState) {
    let (_, view_height) = &game_state.view_port;

    let pos = self.position;
    let (_, height) = self.size;

    if game_state.keyboard_state.contains(&self.controls.up) {
      if (pos.y() - PLAYER_VELOCITY) > 0 {
        self.move_by((0, -25));
      } else {
        self.set_position((pos.x, 0));
      }
    }

    if game_state.keyboard_state.contains(&self.controls.down) {
      if (pos.y() + PLAYER_VELOCITY) < (view_height - height) as i32 {
        self.move_by((0, 25));
      } else {
        self.set_position((pos.x(), (view_height - height) as i32));
      }
    }
  }

  fn render(&self, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(self.color);
    canvas
      .fill_rect(self.as_rect())
      .expect("Unable to draw player to canvas");
  }
}
