extern crate sdl2;

use std::collections::HashSet;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use sdl2::render::Canvas;
use sdl2::video::Window;

mod lib;
use lib::entities::{Ball, Entity, Player};
use lib::text;
use lib::{Controls, GameState};

const DEFAULT_VIEW_SIZE: (u32, u32) = (800, 600);
// Distance from walls
const PLAYER_X: i32 = 100;

static FONT_PATH: &str = "src/assets/font.ttf";

const PLAYER_1_CONTROLS: Controls = Controls {
  up: Scancode::W,
  down: Scancode::S,
};

const PLAYER_2_CONTROLS: Controls = Controls {
  up: Scancode::Up,
  down: Scancode::Down,
};

fn draw_dotted_line(canvas: &mut Canvas<Window>) {
  let (view_width, view_height) = canvas.window().drawable_size();

  let width = 5;
  let height = 20;

  let rect_x = ((view_width / 2) - (width / 2)) as i32;

  let mut rect = Rect::new(rect_x, 0, width, height);

  while (rect.y() as u32 + rect.height()) < view_height {
    canvas.fill_rect(rect).expect("Unable to draw dotted line");
    rect = Rect::new(rect_x, rect.y() + (height * 2) as i32, width, height);
  }
}

fn create_canvas(context: &sdl2::Sdl) -> Canvas<Window> {
  let video_subsystem = context.video().expect("Couldn't get SDL video subsystem");
  let (view_width, view_height) = DEFAULT_VIEW_SIZE;

  let window = video_subsystem
    .window("pongrs", view_width, view_height)
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
  let ttf_context = sdl2::ttf::init().expect("TTF context initialization failed");

  let mut canvas = create_canvas(&sdl_context);
  let texture_creator = canvas.texture_creator();

  let font = ttf_context
    .load_font(Path::new(&FONT_PATH), 16)
    .expect("Unable to load font");

  let mut event_pump = sdl_context
    .event_pump()
    .expect("Failed to get SDL event pump");

  let (view_width, view_height) = canvas.window().drawable_size();
  let view_port = (view_width, view_height);
  let view_port_mid = Point::new((view_width / 2) as i32, (view_height / 2) as i32);
  let view_scale = (view_width / DEFAULT_VIEW_SIZE.0) as i32;

  let mut player1 = Player::new(PLAYER_1_CONTROLS);
  let mut player2 = Player::new(PLAYER_2_CONTROLS);
  let mut ball = Ball::new();

  let texture_0 = text::create_texture_from_string(&texture_creator, font, String::from("0"));

  // Initialize entities
  let player1_x = PLAYER_X * view_scale;
  let player2_x = view_width as i32 - (PLAYER_X * view_scale);

  player1.set_center((player1_x, view_port_mid.y()));
  player2.set_center((player2_x, view_port_mid.y()));

  let mut game_state = GameState {
    view_port,
    keyboard_state: HashSet::new(),
    playing: false,
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

    if !game_state.playing {
      ball.set_velocity((0, 0));
      ball.set_center((view_port_mid.x(), view_port_mid.y()));
    }

    if !game_state.playing && game_state.keyboard_state.contains(&Scancode::Space) {
      ball.starting_velocity();
      game_state.playing = true;
    }

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    player1.update(&mut game_state);
    player2.update(&mut game_state);
    ball.update(&mut game_state);

    player1.render(&mut canvas);
    player2.render(&mut canvas);
    ball.render(&mut canvas);

    draw_dotted_line(&mut canvas);

    text::render_string_texture(&mut canvas, &texture_0, (0, 0));

    canvas.present();

    sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
