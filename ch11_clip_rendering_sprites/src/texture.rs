use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub struct Sprite<'a> 
{
  texture: &'a Texture<'a>, 
  src_rect: Rect, // Source rectangle defining the sprite's portion in the texture
  name: String
}

impl<'a> Sprite<'a> 
{
  pub fn new(texture: &'a Texture<'a>, src_rect: Rect, name: String) -> Sprite<'a> 
  {
    Sprite { texture, src_rect, name }
  }

  pub fn render(&self, canvas: &mut WindowCanvas, x: i32, y: i32) 
  {
    let dest_rect = Rect::new(x, y, self.src_rect.width(), self.src_rect.height());
    canvas.copy(self.texture, self.src_rect, dest_rect).unwrap();
  }
}
