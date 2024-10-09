extern crate sdl2; 

mod errors;
mod texture;
mod sprite;

use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::IntegerOrSdlError;
use sdl2::image::{Sdl2ImageContext, InitFlag};
use sdl2::VideoSubsystem;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use errors::*;
use sprite::create_sprites;
use sprite::load_sprites_from_json;
use texture::TextureManager;


// To group initializations, mainly for readability: I may group them differently in the future.
// ... maybe in a single struct with the different contexts ...
// Now also initalizing the eventpump and the canvas here...
fn init_sdl2(win_title: &str, win_width: u32, win_height: u32) 
    -> Result<(Sdl, Sdl2ImageContext, VideoSubsystem, EventPump, Canvas<Window>), String>
{
  let sdl_context = sdl2::init()?;
  sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1"); // for pixel linear interpolation
  let image_context = sdl2::image::init(InitFlag::PNG)?;
  let video_subsystem = sdl_context.video()?;
  let window = video_subsystem.window(win_title, win_width, win_height)
    .position_centered().build()
    .map_err(|e| e.to_string())?;
    
  let event_pump = sdl_context.event_pump()?;
  
  // The main object to render textures on (<=> SDL_CreateRenderer)
  let canvas : Canvas<Window> = window.into_canvas()
    .present_vsync().build()
    .map_err(|e| {
        match e {
          IntegerOrSdlError::IntegerOverflows(msg, val) => {
            format!("int overflow {}, val: {}", msg, val)
          }
          IntegerOrSdlError::SdlError(msg) => { 
            format!("SDL error: {}", msg) 
          }
        }
    })?;
    
  Ok((sdl_context, image_context, video_subsystem, event_pump, canvas))
    // no need to return the window anymore, it is held by the canvas
}

/////////////////////////////////////////////////////////

fn main() -> Result<(), String> 
{
  const WINDOW_WIDTH: u32 = 1000;
  const WINDOW_HEIGHT: u32 = 600;
  
  let (_sdl_context, _image_ctx, _video_subsystem, mut event_pump, mut canvas) 
      = init_sdl2("MatouTest", WINDOW_WIDTH, WINDOW_HEIGHT)
          .unwrap_or_else(|e| { prompt_err_and_panic("SDL initialization error", &e, None); });

  let texture_creator = canvas.texture_creator();
  let mut texture_manager = TextureManager::new();
  let sprite_data = load_sprites_from_json("data/meta.json");
  let sprites = create_sprites(&texture_creator, sprite_data, &mut texture_manager);

  canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF)); // white, won't change this time
  
  'game : loop 
  {
    for event in event_pump.poll_iter() 
    {
      match event 
      {
        Event::Quit {..} => { break 'game; },
        _ => {}
      }
    }
    
    canvas.clear();
    
    for i in 0..sprites.len() {
      sprites[i].render(&mut canvas, (100*i) as i32, 50);
    }
    
    canvas.present(); 
  }
	
  Ok(())
}
