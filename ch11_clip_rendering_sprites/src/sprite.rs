use crate::errors::*;
use crate::texture::TextureManager;

use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use std::rc::Rc;

pub struct Sprite<'a> 
{
  texture: Rc<Texture<'a>>,
  src_rect: Rect, // Source rectangle defining the sprite's portion in the texture
  name: String
}

impl<'a> Sprite<'a> 
{
  pub fn new(texture: Rc<Texture<'a>>, src_rect: Rect, name: String) -> Sprite<'a> 
  {
    Sprite { texture, src_rect, name }
  }

  pub fn render(&self, canvas: &mut WindowCanvas, x: i32, y: i32) 
  {
    let dest_rect = Rect::new(x, y, self.src_rect.width(), self.src_rect.height());
    canvas.copy(&self.texture, self.src_rect, dest_rect).unwrap();
  }
}

// Represent deserialized sprite data
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SpriteData 
{
  name: String,
  x: i32,
  y: i32,
  w: u32,
  h: u32
}

#[derive(Deserialize)]
pub struct SpriteSheetData 
{
  spritesheet: String, // path of the png
  sprites: Vec<SpriteData>
}

// Deserialize sprite data from json
use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;

pub fn load_sprites_from_json(file_path: &str) -> SpriteSheetData 
{
  let file = File::open(file_path)
    .unwrap_or_else(|err| { prompt_err_and_panic("load_sprites_from_json failed(open)", &err.to_string(), None); });
  let reader = BufReader::new(file);
  let sprite_data: SpriteSheetData = from_reader(reader)
    .unwrap_or_else(|err| { prompt_err_and_panic("load_sprites_from_json failed(read)", &err.to_string(), None); });
  
  sprite_data
}


pub fn create_sprites<'a>(
  texture_creator: &'a TextureCreator<WindowContext>,
  sprite_data: SpriteSheetData,
  texture_manager: &mut TextureManager<'a>) -> Vec<Sprite<'a>> 
{
  let tex = {
    texture_manager.load_texture(texture_creator, &sprite_data.spritesheet, None) // TODO color keying
  };
  
  sprite_data.sprites
    .into_iter()
    .map(|data| 
    {
      let src_rect = Rect::new(data.x, data.y, data.w, data.h);
      Sprite::new(tex.clone(), src_rect, data.name)
    })
    .collect()
}
