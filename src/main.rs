extern crate sdl2;
mod lib;

use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use lib::entities::{Entity, Player};

fn create_canvas(context: &sdl2::Sdl) -> Canvas<Window> {
  let video_subsystem = context.video().expect("Couldn't get SDL video subsystem");

  let window = video_subsystem
    .window("rust-sdl2 demo: Video", 800, 600)
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

  let mut player = Player::new(&mut canvas, &texture_creator);
  player.set_position(0, 128);

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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    player.render(&mut canvas);

    canvas.present();

    sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
