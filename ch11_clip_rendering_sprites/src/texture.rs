use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::pixels::Color;

use std::collections::HashMap;
use std::rc::Rc;

use crate::prompt_err_and_panic;


pub struct TextureManager<'a> {
  textures: HashMap<String, Rc<Texture<'a>>>  // HashMap for caching textures by file path
}

impl<'a> TextureManager<'a>
{
  pub fn new() -> TextureManager<'a>
  {
    TextureManager { textures: HashMap::new() }
  }

  pub fn load_texture(&mut self, 
    texture_creator: &'a TextureCreator<WindowContext>,
    img_path: &str, 
    color_key: Option<Color>) -> Rc<Texture<'a>>
  {
    if !self.textures.contains_key(img_path) 
    {
      use sdl2::surface::Surface;
      use sdl2::image::LoadSurface;
      use std::path::Path;
      let mut s = Surface::from_file(Path::new(img_path))
        .unwrap_or_else(|err| { prompt_err_and_panic("img_load_color_key failed", &err, None); });
      
      match color_key 
      {
        Some(col) => 
        { 
          s.set_color_key(true, col)
          .unwrap_or_else(|err| { prompt_err_and_panic("img_load_color_key(set_color_key) failed", &err, None); }); 
        },
        None => {}
      }
        
      let tex = s.as_texture(texture_creator)
        .unwrap_or_else(|err| { 
          prompt_err_and_panic("img_load_color_key(as_texture) failed", &err.to_string(), None); });
      self.textures.insert(img_path.to_string(), Rc::new(tex));
    }
    Rc::clone(self.textures.get(img_path).unwrap())
  }
}
