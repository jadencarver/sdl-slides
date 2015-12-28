extern crate sdl2;

pub struct Timeline {
  frame: u32,
  pub sprites: Vec<Sprite>
}

impl Timeline {
  pub fn new() -> Timeline {
    Timeline { frame: 0, sprites: vec!() }
  }

  pub fn tick(&mut self) {
    self.frame += 1;
  }

  pub fn render(&self, mut renderer: &mut sdl2::render::Renderer) {
    for sprite in self.sprites.iter() { sprite.render(&mut renderer, self.frame + sprite.frame_offset) }
  }

  pub fn add(&mut self, mut sprite: Sprite) {
    sprite.frame_offset = self.frame;
    self.sprites.push(sprite)
  }
}

pub struct Sprite {
  pub texture: sdl2::render::Texture,
  pub rect: sdl2::rect::Rect,
  frame_offset: u32
}

impl Sprite {
  pub fn new(texture: sdl2::render::Texture, rect: sdl2::rect::Rect) -> Sprite {
    Sprite { texture: texture, rect: rect, frame_offset: 0 }
  }
  fn render(&self, renderer: &mut sdl2::render::Renderer, frame: u32) {
    renderer.copy_ex(&self.texture, None, Some(self.rect), frame as f64, None, (false, false));
  }
}