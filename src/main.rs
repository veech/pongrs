extern crate sdl2;

use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;

use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

mod lib;
use lib::entities::{Controls, Entity, GameState, Player};
use lib::Size;

const VIEW_SIZE: Size = Size {
  height: 600,
  width: 800,
};

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
    .window("rust-sdl2 demo: Video", VIEW_SIZE.width, VIEW_SIZE.height)
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
  let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

  let mut event_pump = sdl_context
    .event_pump()
    .expect("Failed to get SDL event pump");

  let mut player1 = Player::new(&mut canvas, &texture_creator, PLAYER_1_CONTROLS);
  player1.set_position(0, 128);

  let mut player2 = Player::new(&mut canvas, &texture_creator, PLAYER_2_CONTROLS);
  player2.set_position(((VIEW_SIZE.width * 2) - player2.size().width) as i32, 128);

  'game: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'game,
        Event::Quit { .. } => break 'game,
        _ => {}
      }
    }

    let keyboard_state: HashSet<Scancode> =
      event_pump.keyboard_state().pressed_scancodes().collect();

    let game_state = GameState {
      view_port: VIEW_SIZE,
      keyboard_state,
    };

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    player1.update(&game_state);
    player2.update(&game_state);

    player1.render(&mut canvas);
    player2.render(&mut canvas);

    canvas.present();

    sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
