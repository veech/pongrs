use sdl2::render::Canvas;
use sdl2::video::Window;

use super::GameState;

pub trait Entity {
  fn update(&mut self, game_state: &mut GameState);
  fn render(&self, canvas: &mut Canvas<Window>);
}

mod ball;
pub use self::ball::Ball;

mod player;
pub use self::player::Player;
