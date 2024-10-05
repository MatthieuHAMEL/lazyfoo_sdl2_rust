extern crate sdl2; 

use sdl2::surface::Surface;
use sdl2::messagebox::*;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::collections::HashMap;
use std::path::Path;

fn prompt_err_and_panic(message: &str, error: &str, window: Option<&Window>) -> ! 
{
  // (in a real application I'd log the error before trying to prompt the msg box, cf. chapter 2 comment)
  show_simple_message_box(
    MessageBoxFlag::ERROR,
    "FATAL ERROR",
    &format!("{}: {}", message, error),
    window,
  ).unwrap(); 

  panic!("{}: {}", message, error);
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum KeyPressSurface 
{
  Default,
  Up,
  Down,
  Left,
  Right,
}

// This struct and its impl are an alternative to the loadMedia() function from the tutorial. Also avoids that global array indexed by an enum
struct MySurfaces {
  surfaces: HashMap<KeyPressSurface, Surface<'static>>,
}

impl MySurfaces 
{
  fn new() -> Result<Self, String> 
  {
    let mut surfaces = HashMap::new();

    surfaces.insert(KeyPressSurface::Default, MySurfaces::load_surface("data/press.bmp"));
    surfaces.insert(KeyPressSurface::Up, MySurfaces::load_surface("data/up.bmp"));
    surfaces.insert(KeyPressSurface::Down, MySurfaces::load_surface("data/down.bmp"));
    surfaces.insert(KeyPressSurface::Left, MySurfaces::load_surface("data/left.bmp"));
    surfaces.insert(KeyPressSurface::Right, MySurfaces::load_surface("data/right.bmp"));

    Ok(MySurfaces { surfaces })
  }

  // Prompts a message box and make the program panic in case of failure
  fn load_surface(file_path: &str) -> Surface<'static> 
  {
    Surface::load_bmp(Path::new(file_path))
      .unwrap_or_else(|err| { prompt_err_and_panic("load_surface failed", &err, None); })
  }

  fn get_surface(&self, key: KeyPressSurface) -> &Surface {
    &self.surfaces[&key]
  }
}


fn main() -> Result<(), String> 
{    
  let sdl_context = sdl2::init()
		.unwrap_or_else(|e| {
      prompt_err_and_panic("SDL2 Init Error", &e, None);
    });
        
  let video_subsystem = sdl_context.video()
    .unwrap_or_else(|e| { prompt_err_and_panic("Video Subsystem Error", &e, None); });
        
  let window = video_subsystem.window("MatouTest", 1000, 600)
    .position_centered().build()
    .map_err(|e| e.to_string())
    .unwrap_or_else(|e| { prompt_err_and_panic("Window Creation Error", &e, None); });
    
	let mut event_pump = sdl_context.event_pump()
		.unwrap_or_else(|e| { prompt_err_and_panic("SDL, no event pump", &e, None); });
  
  let surfaces = MySurfaces::new()?;
  let mut current_surface = surfaces.get_surface(KeyPressSurface::Default);
	
	'game : loop 
  {
		for event in event_pump.poll_iter() 
    {
			match event 
      {
				Event::Quit {..} => { break 'game; },
        Event::KeyDown { keycode, .. } => 
        {
          match keycode 
          {
            Some(Keycode::Up) => { current_surface = surfaces.get_surface(KeyPressSurface::Up); }
            Some(Keycode::Down) => { current_surface = surfaces.get_surface(KeyPressSurface::Down); }
            Some(Keycode::Left) => { current_surface = surfaces.get_surface(KeyPressSurface::Left); }
            Some(Keycode::Right) => { current_surface = surfaces.get_surface(KeyPressSurface::Right); }
            _ => { current_surface = surfaces.get_surface(KeyPressSurface::Default); }
          }
        },
        _ => {}
      }
    }
    
    let mut wsuf = window.surface(&event_pump).unwrap();
    current_surface.blit(None, &mut wsuf, None)?; 
    wsuf.update_window()?;
	}
	
  Ok(())
}
