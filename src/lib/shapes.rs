use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use super::{Size, Vec2};

const DEFAULT_POSITION: Vec2 = Vec2 { x: 0, y: 0 };

pub struct Square<'a> {
  pub size: Size,
  pub position: Vec2,

  color: Option<Color>,
  texture: Option<Texture<'a>>,
}

impl<'a> Square<'a> {
  pub fn new(height: u32, width: u32) -> Square<'a> {
    Square {
      size: Size { height, width },
      position: DEFAULT_POSITION,

      color: None,
      texture: None,
    }
  }

  pub fn draw_to_canvas(&self, canvas: &mut Canvas<Window>) {
    let pos = self.position;
    let size = self.size;

    let width = size.width;
    let height = size.height;

    match &self.texture {
      Some(texture) => {
        canvas
          .copy(&texture, None, Rect::new(pos.x, pos.y, width, height))
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

    match texture_creator.create_texture_target(None, self.size.width, self.size.height) {
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
