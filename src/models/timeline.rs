extern crate sdl2;
use models::sprite::Sprite;

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