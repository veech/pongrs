use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::{Controls, GameState};

const PLAYER_COLOR: Color = Color::RGB(255, 255, 255);
const PLAYER_VELOCITY: i32 = 25;
const PLAYER_WIDTH: u32 = 20;
const PLAYER_HEIGHT: u32 = 150;

const BALL_COLOR: Color = Color::RGB(255, 255, 255);
const BALL_INITIAL_VELOCITY: i32 = 10;
const BALL_SIZE: (u32, u32) = (16, 16);

pub trait Entity {
  fn update(&mut self, game_state: &mut GameState);
  fn render(&self, canvas: &mut Canvas<Window>);
}

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

  pub fn move_by(&mut self, delta: (i32, i32)) {
    let (dx, dy) = delta;
    let pos = self.position;

    self.position = Point::new(pos.x + dx, pos.y + dy);
  }

  pub fn size(&self) -> (u32, u32) {
    self.size
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

pub struct Ball {
  size: (u32, u32),
  color: Color,
  position: Point,
  velocity: Point,
}

impl Ball {
  pub fn new() -> Ball {
    Ball {
      position: Point::new(0, 0),
      size: BALL_SIZE,
      color: BALL_COLOR,

      velocity: Point::new(BALL_INITIAL_VELOCITY, BALL_INITIAL_VELOCITY),
    }
  }

  pub fn set_position(&mut self, pos: (i32, i32)) {
    let (x, y) = pos;
    self.position = Point::new(x, y);
  }

  pub fn size(&self) -> (u32, u32) {
    self.size
  }

  pub fn as_rect(&self) -> Rect {
    let pos = self.position;
    let (width, height) = self.size;

    Rect::new(pos.x(), pos.y(), width, height)
  }

  pub fn collides_with(&self, rect: Rect) -> bool {
    let ball_rect = self.as_rect();

    return ball_rect.x() < rect.x() + rect.width() as i32
      && ball_rect.x() + ball_rect.width() as i32 > rect.x()
      && ball_rect.y() < rect.y() + rect.height() as i32
      && ball_rect.y() + ball_rect.height() as i32 > rect.y();
  }
}

impl Entity for Ball {
  fn update(&mut self, game_state: &mut GameState) {
    let (view_width, view_height) = game_state.view_port;

    let pos = self.position;
    let vel = self.velocity;

    let (width, height) = self.size;

    self.position = Point::new(pos.x + vel.x, pos.y + vel.y);

    for rect in game_state.player_rects.iter() {
      if self.collides_with(*rect) {
        self.velocity.x = -vel.x;
      }
    }

    if self.position.x() >= (view_width - width) as i32 {
      let (p1_points, p2_points) = game_state.player_points;
      game_state.player_points = (p1_points + 1, p2_points);

      self.velocity.x = -vel.x;
    };

    if self.position.x() <= 0 as i32 {
      let (p1_points, p2_points) = game_state.player_points;
      game_state.player_points = (p1_points, p2_points + 1);

      self.velocity.x = -vel.x;
    };

    if self.position.y() >= (view_height - height) as i32 || self.position.y() <= 0 as i32 {
      self.velocity.y = -vel.y;
    };
  }

  fn render(&self, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(self.color);
    canvas
      .fill_rect(self.as_rect())
      .expect("Unable to draw ball to canvas");
  }
}
