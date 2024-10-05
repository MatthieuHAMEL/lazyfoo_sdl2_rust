extern crate sdl2; 

use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::surface::Surface;
use sdl2::messagebox::*;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

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

// To group initializations, mainly for readability: I may group them differently in the future.
fn init_sdl2(win_title: &str, win_width: u32, win_height: u32) -> Result<(Sdl, VideoSubsystem, Window), String>
{
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let window = video_subsystem.window(win_title, win_width, win_height)
    .position_centered().build()
    .map_err(|e| e.to_string())?;
    
  Ok((sdl_context, video_subsystem, window))
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

// Alternative to the loadMedia() function from the tutorial. Also avoids global variables.
struct MySurfaces {
  surfaces: HashMap<KeyPressSurface, Surface<'static>>,
}

impl MySurfaces 
{
  fn new(format: PixelFormatEnum) -> Result<Self, String> 
  {
    let mut surfaces = HashMap::new();

    surfaces.insert(KeyPressSurface::Default, MySurfaces::load_surface("data/press.bmp", format));
    surfaces.insert(KeyPressSurface::Up, MySurfaces::load_surface("data/up.bmp", format));
    surfaces.insert(KeyPressSurface::Down, MySurfaces::load_surface("data/down.bmp", format));
    surfaces.insert(KeyPressSurface::Left, MySurfaces::load_surface("data/left.bmp", format));
    surfaces.insert(KeyPressSurface::Right, MySurfaces::load_surface("data/right.bmp", format));

    Ok(MySurfaces { surfaces })
  }

  // Prompt a message box and make the program panic in case of failure
  // Now it takes a format in input so that the returned surface is directly correctly formatted
  // ... for example to the window surface format.
  fn load_surface(bmp_path: &str, format: PixelFormatEnum) -> Surface<'static> 
  {
    Surface::load_bmp(Path::new(bmp_path))
      .unwrap_or_else(|err| { prompt_err_and_panic("load_surface(load_bmp) failed", &err, None); })
      .convert_format(format)
      .unwrap_or_else(|err| { prompt_err_and_panic("load_surface(convert_format) failed", &err, None); })
  }

  fn get_surface(&self, key: KeyPressSurface) -> &Surface {
    &self.surfaces[&key]
  }
}

fn main() -> Result<(), String> 
{
  const WINDOW_WIDTH: u32 = 1000;
  const WINDOW_HEIGHT: u32 = 600;
  
  let (sdl_context, _video_subsystem, window) = init_sdl2("MatouTest", WINDOW_WIDTH, WINDOW_HEIGHT)
    .unwrap_or_else(|e| { prompt_err_and_panic("SDL initialization error", &e, None); });
  
  let mut event_pump = sdl_context.event_pump()
    .unwrap_or_else(|e| { prompt_err_and_panic("SDL, no event pump", &e, None); });
  
  let wsuf_format = window.surface(&event_pump).unwrap().pixel_format_enum();
  let surfaces = MySurfaces::new(wsuf_format)?;
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
    
    // Stretch the current surface to the window size !
    // Provided 'surfaces' has been initialized with the window surface pixel format, the surfaces we manipulate are optimized now. 
    let mut wsuf = window.surface(&event_pump).unwrap();
    current_surface.blit_scaled(None, &mut wsuf, Some(Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)))?; 
    wsuf.update_window()?;
  }
	
  Ok(())
}
