extern crate sdl2; 

use sdl2::Sdl;
use sdl2::image::{Sdl2ImageContext, InitFlag};
use sdl2::VideoSubsystem;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

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
struct MySurfaces {
  surfaces: HashMap<KeyPressSurface, Surface<'static>>,
}

impl MySurfaces 
{
  fn new(format: PixelFormatEnum) -> Result<Self, String> 
  {
    let mut surfaces = HashMap::new();

    surfaces.insert(KeyPressSurface::Default, MySurfaces::img_load("data/press.png", format));
    surfaces.insert(KeyPressSurface::Up, MySurfaces::img_load("data/up.png", format));
    surfaces.insert(KeyPressSurface::Down, MySurfaces::img_load("data/down.png", format));
    surfaces.insert(KeyPressSurface::Left, MySurfaces::img_load("data/left.png", format));
    surfaces.insert(KeyPressSurface::Right, MySurfaces::img_load("data/right.png", format));

    Ok(MySurfaces { surfaces })
  }

  // Prompt a message box and make the program panic in case of failure
  // Now it takes a format in input so that the returned surface is directly correctly formatted
  // ... for example to the window surface format.
  #[allow(dead_code)] // At this point, BMPs are dead, but I'm leaving the function for nostalgia's sake.  
  fn load_surface_bmp(bmp_path: &str, format: PixelFormatEnum) -> Surface<'static> 
  {
    Surface::load_bmp(Path::new(bmp_path))
      .unwrap_or_else(|err| { prompt_err_and_panic("load_surface(load_bmp) failed", &err, None); })
      .convert_format(format)
      .unwrap_or_else(|err| { prompt_err_and_panic("load_surface(convert_format) failed", &err, None); })
  }
  
  // sdl2::image::init should have been called before, with the InitFlags corresponding to the wanted 
  // image type. [nota bene, it works without the initialization though!]
  fn img_load(img_path: &str, format: PixelFormatEnum) -> Surface
  {
    use crate::sdl2::image::LoadSurface;
    Surface::from_file(Path::new(img_path))
      .unwrap_or_else(|err| { prompt_err_and_panic("img_load_surface failed", &err, None); })
      .convert_format(format)
      .unwrap_or_else(|err| { prompt_err_and_panic("img_load_surface(convert_format) failed", &err, None); })
  }

  fn get_surface(&self, key: KeyPressSurface) -> &Surface {
    &self.surfaces[&key]
  }
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

/////////////////////////////////////////////////////////

fn main() -> Result<(), String> 
{
  const WINDOW_WIDTH: u32 = 1000;
  const WINDOW_HEIGHT: u32 = 600;
  
  let (sdl_context, _image_ctx, _video_subsystem, window) = init_sdl2("MatouTest", WINDOW_WIDTH, WINDOW_HEIGHT)
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

//////////////////////////////////////////////////////////////////

// Unit tests of img_load (I put everything in main.rs ... for now)
#[cfg(test)]
mod tests 
{
  use super::*;
  use sdl2::pixels::PixelFormatEnum;

  #[test]
  fn test_valid_image_load() {    // I should actually test with every pixel format I'd need.
    let result = MySurfaces::img_load("data/right.png", PixelFormatEnum::RGBA8888);
    assert!(result.width() > 0 && result.height() > 0, "Wrong dimensions!");
  }

  #[test]
  #[should_panic]
  fn test_non_existent_image_load() {
    // This test expects a panic (in cfg(not(test)) there would also be my message box
    MySurfaces::img_load("non_existent_image.png", PixelFormatEnum::RGBA8888);
  }

  #[test]
  #[should_panic]
  fn test_invalid_image_format() {
    // This is a text file, despite appearances
    MySurfaces::img_load("data/test/invalid_image.png", PixelFormatEnum::RGBA8888);
  }

  #[test]
  #[should_panic]
  fn test_unsupported_pixel_format_conversion() {
    // Try to convert to a stupid pixel format
    MySurfaces::img_load("data/right.png", PixelFormatEnum::Unknown);
  }
}
