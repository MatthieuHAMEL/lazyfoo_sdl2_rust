extern crate sdl2; 

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use sdl2::messagebox::*;
use sdl2::video::Window;
use sdl2::surface::Surface;

// Adding message boxes for "really fatal" errors.
fn prompt_error_and_panic(message: &str, error: &str, window: Option<&Window>) -> ! 
{
  show_simple_message_box(
      MessageBoxFlag::ERROR,
      "FATAL ERROR",
      &format!("{}: {}", message, error),
      window,
  ).unwrap(); // note that if the box didn't work we won't get the panic in stderr with the right message
        // So in a real application - TODO - I'd be
        // logging the error in a file before trying to create a message box 
  
  panic!("{}: {}", message, error);
}

fn main() -> Result<(), String> 
{  
  let sdl_context = sdl2::init()
    .unwrap_or_else(|e| {
      prompt_error_and_panic("SDL2 Init Error", &e, None);
    });
      
  let video_subsystem = sdl_context.video()
    .unwrap_or_else(|e| {
      prompt_error_and_panic("Video Subsystem Error", &e, None);
    });
      
  let window = video_subsystem.window("MatouTest", 1000, 600)
    .position_centered().build()
    .map_err(|e| e.to_string())
    .unwrap_or_else(|e| {
      prompt_error_and_panic("Window Creation Error", &e, None);
    });

  // window.surface() borrows the event pump to prevent the window to be 
  // resized during its lifetime (if it was resized, we'd have a dangling 
  // pointer to the surface, in C). Here we don't have an event loop so...
  let fake_event_pump = sdl_context.event_pump().unwrap();
  let mut wsuf = window.surface(&fake_event_pump).unwrap();
	let surface = Surface::load_bmp(&Path::new("data/test.bmp"))
		.unwrap_or_else(|e| {
			prompt_error_and_panic("Couldn't load BMP", &e, None);
		});
	
  surface.blit(None, &mut wsuf, None)?; 
  wsuf.update_window()?;
	
	// Sleep 5s to see something on the screen. Of course it will freeze the window
  sleep(Duration::new(5, 0));
	
  Ok(())
}
