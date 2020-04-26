use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use super::shapes;
use super::{Controls, GameState, Size, Vec2};

const PLAYER_COLOR: Color = Color::RGB(255, 255, 255);
const PLAYER_VELOCITY: i32 = 25;
const PLAYER_SIZE: Size = Size {
  height: 150,
  width: 20,
};

const BALL_COLOR: Color = Color::RGB(255, 255, 255);
const BALL_INITIAL_VELOCITY: i32 = 5;
const BALL_SIZE: Size = Size {
  height: 16,
  width: 16,
};

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
    let mut square = shapes::Square::new(PLAYER_SIZE.height, PLAYER_SIZE.width);
    square.set_color(canvas, texture_creator, PLAYER_COLOR);

    Player { controls, square }
  }

  pub fn position(&self) -> Vec2 {
    self.square.position
  }

  pub fn size(&self) -> Size {
    self.square.size
  }

  pub fn set_position(&mut self, x: i32, y: i32) {
    self.square.position = Vec2 { x, y };
  }

  pub fn move_by(&mut self, dx: i32, dy: i32) {
    let pos = self.square.position;
    self.square.position = Vec2 {
      x: pos.x + dx,
      y: pos.y + dy,
    };
  }
}

impl Entity for Player<'_> {
  fn update(&mut self, game_state: &GameState) {
    let view_port = &game_state.view_port;

    let pos = self.position();
    let size = self.size();

    if game_state.keyboard_state.contains(&self.controls.up) {
      if (pos.y - PLAYER_VELOCITY) > 0 {
        self.move_by(0, -25);
      } else {
        self.set_position(pos.x, 0);
      }
    }

    if game_state.keyboard_state.contains(&self.controls.down) {
      if (pos.y + PLAYER_VELOCITY) < (view_port.height - size.height) as i32 {
        self.move_by(0, 25);
      } else {
        self.set_position(pos.x, (view_port.height - size.height) as i32);
      }
    }
  }

  fn render(&self, canvas: &mut Canvas<Window>) {
    self.square.draw_to_canvas(canvas);
  }
}

pub struct Ball<'a> {
  square: shapes::Square<'a>,
  velocity: Vec2,
}

impl<'a> Ball<'a> {
  pub fn new(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
  ) -> Ball<'a> {
    let mut square = shapes::Square::new(BALL_SIZE.height, BALL_SIZE.width);
    square.set_color(canvas, texture_creator, BALL_COLOR);

    Ball {
      square,
      velocity: Vec2 {
        x: BALL_INITIAL_VELOCITY,
        y: BALL_INITIAL_VELOCITY,
      },
    }
  }

  pub fn position(&self) -> Vec2 {
    self.square.position
  }

  pub fn size(&self) -> Size {
    self.square.size
  }

  pub fn set_position(&mut self, pos: Vec2) {
    self.square.position = Vec2 { x: pos.x, y: pos.y };
  }
}

impl Entity for Ball<'_> {
  fn update(&mut self, game_state: &GameState) {
    let pos = self.position();
    let vel = self.velocity;

    let new_pos = Vec2 {
      x: pos.x + vel.x,
      y: pos.y + vel.y,
    };

    if new_pos.x >= (game_state.view_port.width - self.size().width) as i32 {
      self.velocity.x = -vel.x
    };

    if new_pos.x <= 0 as i32 {
      self.velocity.x = -vel.x
    };

    if new_pos.y >= (game_state.view_port.height - self.size().height) as i32 {
      self.velocity.y = -vel.y
    };

    if new_pos.y <= 0 as i32 {
      self.velocity.y = -vel.y
    };

    self.square.position = new_pos;
  }

  fn render(&self, canvas: &mut Canvas<Window>) {
    self.square.draw_to_canvas(canvas);
  }
}
