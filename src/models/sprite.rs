extern crate sdl2;

pub struct Sprite {
  pub texture: sdl2::render::Texture,
  pub rect: sdl2::rect::Rect,
  pub frame_offset: u32
}

impl Sprite {
  pub fn new(texture: sdl2::render::Texture, rect: sdl2::rect::Rect) -> Sprite {
    Sprite { texture: texture, rect: rect, frame_offset: 0 }
  }
  pub fn render(&self, renderer: &mut sdl2::render::Renderer, frame: u32) {
    renderer.copy_ex(&self.texture, None, Some(self.rect), frame as f64, None, (false, false));
  }
}