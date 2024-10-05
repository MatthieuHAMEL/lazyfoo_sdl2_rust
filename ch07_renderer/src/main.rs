extern crate sdl2; 

use sdl2::Sdl;
use sdl2::IntegerOrSdlError;
use sdl2::image::{Sdl2ImageContext, InitFlag};
use sdl2::VideoSubsystem;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;

use std::collections::HashMap;
use std::path::Path;

#[cfg(test)]
fn prompt_err_and_panic(message: &str, error: &str, _window: Option<&Window>) -> ! 
{
  panic!("{}: {}", message, error);
}

#[cfg(not(test))]
fn prompt_err_and_panic(message: &str, error: &str, window: Option<&Window>) -> ! 
{
  use sdl2::messagebox::*;
  // (in a real application I'd log the error before trying to prompt the msg box, cf. chapter 2 comment)
  show_simple_message_box(
    MessageBoxFlag::ERROR,
    "FATAL ERROR",
    &format!("{}: {}", message, error),
    window,
  ).unwrap(); 

  panic!("{}: {}", message, error);
}

// To group initializations, mainly for readability: I may group them differently in the future.
// ... maybe in a single struct with the different contexts ...
fn init_sdl2(win_title: &str, win_width: u32, win_height: u32) -> Result<(Sdl, Sdl2ImageContext, VideoSubsystem, Window), String>
{
  let sdl_context = sdl2::init()?;
  let image_context = sdl2::image::init(InitFlag::PNG)?;
  let video_subsystem = sdl_context.video()?;
  let window = video_subsystem.window(win_title, win_width, win_height)
    .position_centered().build()
    .map_err(|e| e.to_string())?;
    
  Ok((sdl_context, image_context, video_subsystem, window))
}

// Alternative to the loadMedia() function from the tutorial. Also avoids global variables.
// Lifetime considerations here ; the Texture (borrowed) can't outlive the TextureCreator (owner),
// so neither can the whole struct, otherwise there'd be dangling pointers in the map.
struct MyTextures<'a> {
  textures: HashMap<KeyPress, Texture<'a>>,
}

impl<'a> MyTextures<'a> 
{
  fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> 
  {
    let mut textures = HashMap::new();

    textures.insert(KeyPress::Default, MyTextures::img_load("data/press.png", texture_creator));
    textures.insert(KeyPress::Up, MyTextures::img_load("data/up.png", texture_creator));
    textures.insert(KeyPress::Down, MyTextures::img_load("data/down.png", texture_creator));
    textures.insert(KeyPress::Left, MyTextures::img_load("data/left.png", texture_creator));
    textures.insert(KeyPress::Right, MyTextures::img_load("data/right.png", texture_creator));

    Ok(MyTextures { textures })
  }
  
  // sdl2::image::init should have been called before, with the InitFlags corresponding to the wanted 
  // image type. [nota bene, it works without the initialization though!]
  fn img_load(img_path: &str, texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a>
  {
    use crate::sdl2::image::LoadTexture;
    texture_creator.load_texture(Path::new(img_path))
      .unwrap_or_else(|err| { prompt_err_and_panic("img_load failed", &err, None); })
  }

  fn from_key(&self, key: KeyPress) -> &Texture {
    &self.textures[&key]
  }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum KeyPress
{
  Default,
  Up,
  Down,
  Left,
  Right,
}

/////////////////////////////////////////////////////////

fn main() -> Result<(), String> 
{
  const WINDOW_WIDTH: u32 = 1000;
  const WINDOW_HEIGHT: u32 = 600;
  
  let (sdl_context, _image_ctx, _video_subsystem, window) = init_sdl2("MatouTest", WINDOW_WIDTH, WINDOW_HEIGHT)
    .unwrap_or_else(|e| { prompt_err_and_panic("SDL initialization error", &e, None); });
  
  let mut event_pump = sdl_context.event_pump()
    .unwrap_or_else(|e| { prompt_err_and_panic("SDL, no event pump", &e, None); });
  
  // The main object to render textures on (<=> SDL_CreateRenderer)
  let mut canvas : Canvas<Window> = window.into_canvas()
    .present_vsync().build()
    .unwrap_or_else(|e| {
        let error_msg = match e {
          IntegerOrSdlError::IntegerOverflows(msg, val) => {
            format!("int overflow {}, val: {}", msg, val)
          }
          IntegerOrSdlError::SdlError(msg) => { 
            format!("SDL error: {}", msg) 
          }
        };
        prompt_err_and_panic("SDL, no canvas", &error_msg, None);
    });
  
  // The color used for drawing rectangles and clear operations <=> SDL_SetRenderDrawColor
  canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF));
  
  // The objects that owns the textures created from it.
  let texture_creator = canvas.texture_creator();

  // No more windows surfaces, no more pixel formatting considerations. MySurfaces becomes MyTextures !
  let textures = MyTextures::new(&texture_creator)?;
  let mut current_texture = textures.from_key(KeyPress::Default);
	
  let mut running = true;
  while running 
  {
    for event in event_pump.poll_iter() 
    {
      match event 
      {
        Event::Quit {..} => { running = false },
        Event::KeyDown { keycode, .. } => 
        {
          match keycode 
          {
            Some(Keycode::Up) => { current_texture = textures.from_key(KeyPress::Up); }
            Some(Keycode::Down) => { current_texture = textures.from_key(KeyPress::Down); }
            Some(Keycode::Left) => { current_texture = textures.from_key(KeyPress::Left); }
            Some(Keycode::Right) => { current_texture = textures.from_key(KeyPress::Right); }
            _ => { current_texture = textures.from_key(KeyPress::Default); }
          }
        },
        _ => {}
      }
    }
    
     // <=> SDL_RenderClear
    canvas.clear();
    // <=> SDL_RenderCopy
    // 24/10/05 no need to give the dest rectangle if we want to fill the whole window. It stretches automatically. 
    canvas.copy(&current_texture, None, None)?; 
    // <=> SDL_RenderPresent
    canvas.present(); 
  }
	
  Ok(())
}
