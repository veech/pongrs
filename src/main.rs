extern crate sdl2;

use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;

use sdl2::render::Canvas;
use sdl2::video::Window;

mod lib;
use lib::entities::{Ball, Entity, Player};
use lib::{Controls, GameState};

const DEFAULT_VIEW_SIZE: (u32, u32) = (800, 600);

const PLAYER_1_CONTROLS: Controls = Controls {
  up: Scancode::W,
  down: Scancode::S,
};

const PLAYER_2_CONTROLS: Controls = Controls {
  up: Scancode::Up,
  down: Scancode::Down,
};

fn create_canvas(context: &sdl2::Sdl) -> Canvas<Window> {
  let video_subsystem = context.video().expect("Couldn't get SDL video subsystem");

  let window = video_subsystem
    .window("pongrs", DEFAULT_VIEW_SIZE.0, DEFAULT_VIEW_SIZE.1)
    .position_centered()
    .opengl()
    .build()
    .expect("Failed to create window");

  window
    .into_canvas()
    .target_texture()
    .present_vsync()
    .build()
    .expect("Failed to convert window into canvas")
}

pub fn main() {
  let sdl_context = sdl2::init().expect("SDL initialization failed");

  let mut canvas = create_canvas(&sdl_context);

  let mut event_pump = sdl_context
    .event_pump()
    .expect("Failed to get SDL event pump");

  let (view_width, view_height) = canvas.window().drawable_size();
  let view_port = (view_width, view_height);

  let mut player1 = Player::new(PLAYER_1_CONTROLS);
  player1.set_position((0, 128));

  let mut player2 = Player::new(PLAYER_2_CONTROLS);
  player2.set_position(((view_width - player2.size().0) as i32, 128));

  let mut ball = Ball::new();
  let (ball_width, ball_height) = ball.size();
  ball.set_position((
    ((view_width / 2) - (ball_width / 2)) as i32,
    ((view_height / 2) - (ball_height / 2)) as i32,
  ));

  let mut game_state = GameState {
    view_port,
    keyboard_state: HashSet::new(),
    player_rects: Vec::new(),
    player_points: (0, 0),
  };

  'game: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'game,
        _ => {}
      }
    }

    game_state.keyboard_state = event_pump.keyboard_state().pressed_scancodes().collect();
    game_state.player_rects = vec![player1.as_rect(), player2.as_rect()];

    if game_state.keyboard_state.contains(&Scancode::Escape) {
      break 'game;
    }

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    player1.update(&mut game_state);
    player2.update(&mut game_state);
    ball.update(&mut game_state);

    player1.render(&mut canvas);
    player2.render(&mut canvas);
    ball.render(&mut canvas);

    canvas.present();

    sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
