use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const DEFAULT_POSITION: (i32, i32) = (0, 0);

const PLAYER_HEIGHT: u32 = 150;
const PLAYER_WIDTH: u32 = 20;
const PLAYER_COLOR: Color = Color::RGB(255, 255, 255);

pub trait Entity {
  fn render(&self, canvas: &mut Canvas<Window>);
}

struct Square<'a> {
  pub height: u32,
  pub width: u32,

  pub position: (i32, i32),

  color: Option<Color>,
  texture: Option<Texture<'a>>,
}

impl<'a> Square<'a> {
  pub fn new(height: u32, width: u32) -> Square<'a> {
    Square {
      height,
      width,

      position: DEFAULT_POSITION,

      color: None,
      texture: None,
    }
  }

  pub fn draw_to_canvas(&self, canvas: &mut Canvas<Window>) {
    let (x, y) = self.position;

    let width = self.width;
    let height = self.height;

    match &self.texture {
      Some(texture) => {
        canvas
          .copy(&texture, None, Rect::new(x, y, width, height))
          .expect("Could not copy texture into window");
      }
      None => println!("Cannot render square without color"),
    }
  }

  pub fn set_color(
    &mut self,
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: Color,
  ) {
    self.color = Some(color);

    match texture_creator.create_texture_target(None, self.width, self.height) {
      Ok(mut square_texture) => {
        canvas
          .with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(color);
            texture.clear();
          })
          .expect("Failed to color a texture");

        self.texture = Some(square_texture);
      }
      Err(_e) => self.texture = None,
    }
  }
}

pub struct Player<'a> {
  square: Square<'a>,
}

impl<'a> Player<'a> {
  pub fn new(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
  ) -> Player<'a> {
    let mut square = Square::new(PLAYER_HEIGHT, PLAYER_WIDTH);
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
