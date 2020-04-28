use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_dotted_line(canvas: &mut Canvas<Window>, color: Color) {
  let (view_width, view_height) = canvas.window().drawable_size();

  let width = 5;
  let height = 20;

  let rect_x = ((view_width / 2) - (width / 2)) as i32;

  canvas.set_draw_color(color);

  let mut rect = Rect::new(rect_x, 0, width, height);

  while (rect.y() as u32 + rect.height()) < view_height {
    canvas.fill_rect(rect).expect("Unable to draw dotted line");
    rect = Rect::new(rect_x, rect.y() + (height * 2) as i32, width, height);
  }
}

pub fn create_canvas(context: &sdl2::Sdl, view_size: (u32, u32)) -> Canvas<Window> {
  let video_subsystem = context.video().expect("Couldn't get SDL video subsystem");
  let (view_width, view_height) = view_size;

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
