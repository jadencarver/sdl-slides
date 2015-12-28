extern crate rand;
extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color};
use sdl2::rect::{Rect, Point};
use sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const SCREEN_HALF: i32 = SCREEN_WIDTH / 2;
const SCREEN_MIDDLE: i32 = SCREEN_HEIGHT / 2;

mod models;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem.window("SDL SLIDE", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
    .position_centered().opengl()
    .build().unwrap();

  sdl2_image::init(INIT_PNG | INIT_JPG);

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut renderer = window.renderer().present_vsync().build().unwrap();
  let mut timeline = models::Timeline::new();

  timeline.add(models::Sprite::new(
    renderer.load_texture(Path::new("images/logo.png")).unwrap(),
    Rect::new_unwrap(SCREEN_HALF - 416, SCREEN_MIDDLE - 255, 833, 250)
  ));

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    };
    renderer.set_draw_color(Color::RGB(255,255,255));
    renderer.clear();
    timeline.render(&mut renderer);
    timeline.tick();
    renderer.present();
  }

}