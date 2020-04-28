use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

pub const FONT_WIDTH: u32 = 100;
pub const FONT_HEIGHT: u32 = 200;
const FONT_COLOR: Color = Color::RGB(255, 255, 255);

fn create_texture_from_string<'a>(
  texture_creator: &'a TextureCreator<WindowContext>,
  font: &Font,
  string: String,
) -> Texture<'a> {
  let surface = font
    .render(&string)
    .blended(FONT_COLOR)
    .expect("Unable to create font surface");

  return texture_creator
    .create_texture_from_surface(&surface)
    .expect("Unable to create font texture");
}

pub fn generate_score_textures<'a>(
  texture_creator: &'a TextureCreator<WindowContext>,
  font: &Font,
) -> Vec<Texture<'a>> {
  let score_values: Vec<_> = (0u8..10).collect();

  let score_textures = score_values
    .iter()
    .map(|num| create_texture_from_string(texture_creator, font, num.to_string()))
    .collect();

  return score_textures;
}

pub fn render_string_texture(canvas: &mut Canvas<Window>, string: &Texture, position: (i32, i32)) {
  let (x, y) = position;

  canvas
    .copy(&string, None, Rect::new(x, y, FONT_WIDTH, FONT_HEIGHT))
    .expect("Unable to copy texture");
}
