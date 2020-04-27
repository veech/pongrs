use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::super::GameState;
use super::Entity;

const BALL_COLOR: Color = Color::RGB(255, 255, 255);
const BALL_VELOCITY: i32 = 17;
const BALL_WIDTH: u32 = 15;
const BALL_HEIGHT: u32 = 15;

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
      size: (BALL_WIDTH, BALL_HEIGHT),
      color: BALL_COLOR,

      velocity: Point::new(0, 0),
    }
  }

  pub fn set_center(&mut self, pos: (i32, i32)) {
    let (x, y) = pos;
    let (width, height) = self.size;

    self.position = Point::new(x - (width / 2) as i32, y - (height / 2) as i32);
  }

  pub fn set_velocity(&mut self, vel: (i32, i32)) {
    let (x, y) = vel;
    self.velocity = Point::new(x, y);
  }

  pub fn starting_velocity(&mut self) {
    self.set_velocity((-BALL_VELOCITY, 0));
  }

  pub fn as_rect(&self) -> Rect {
    let pos = self.position;
    let (width, height) = self.size;

    Rect::new(pos.x(), pos.y(), width, height)
  }

  pub fn collides_with(&self, rect: Rect) -> bool {
    let ball_rect = self.as_rect();

    return ball_rect.x() <= rect.x() + rect.width() as i32
      && ball_rect.x() + ball_rect.width() as i32 >= rect.x()
      && ball_rect.y() <= rect.y() + rect.height() as i32
      && ball_rect.y() + ball_rect.height() as i32 >= rect.y();
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
        let ball_center = self.as_rect().center();
        let paddle_center = (*rect).center();

        let y_diff = (ball_center.y() - paddle_center.y()) as f32;
        let x_diff = (ball_center.x() - paddle_center.x()) as f32;

        // Scale down value of slope for better launch angles
        let slope: f32 = (y_diff / (x_diff * 5.0)).abs();

        let dy = y_diff.signum() * BALL_VELOCITY as f32 * (slope / (slope.powf(2.0) + 1.0).sqrt());
        let dx = x_diff.signum() * BALL_VELOCITY as f32 * (1.0 / (slope.powf(2.0) + 1.0).sqrt());

        self.velocity.x = dx.round() as i32;
        self.velocity.y = dy.round() as i32;
      }
    }

    if self.position.x() >= (view_width - width) as i32 {
      let (p1_points, p2_points) = game_state.player_points;
      game_state.player_points = (p1_points + 1, p2_points);
      game_state.playing = false;
    };

    if self.position.x() <= 0 as i32 {
      let (p1_points, p2_points) = game_state.player_points;
      game_state.player_points = (p1_points, p2_points + 1);
      game_state.playing = false;
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
