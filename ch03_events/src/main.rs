extern crate sdl2; 

use sdl2::surface::Surface;
use sdl2::messagebox::*;
use sdl2::video::Window;

use std::path::Path;

fn prompt_error_and_panic(message: &str, error: &str, window: Option<&Window>) -> ! 
{
	// log error here in a file (todo)
	show_simple_message_box(
		MessageBoxFlag::ERROR,
		"WHIP - FATAL ERROR",
		&format!("{}: {}", message, error),
		window,
	).unwrap(); 

	panic!("{}: {}", message, error);
}

fn main() -> Result<(), String> 
{
  let sdl_context = sdl2::init()
		.unwrap_or_else(|e| {
      prompt_error_and_panic("SDL2 Init Error", &e, None);
    });
        
  let video_subsystem = sdl_context.video()
    .unwrap_or_else(|e| { prompt_error_and_panic("Video Subsystem Error", &e, None); });
        
  let window = video_subsystem.window("MatouTest", 1980, 1080)
    .position_centered().build()
    .map_err(|e| e.to_string())
    .unwrap_or_else(|e| { prompt_error_and_panic("Window Creation Error", &e, None); });
    
	let mut event_pump = sdl_context.event_pump()
		.unwrap_or_else(|e| { prompt_error_and_panic("SDL, no event pump", &e, None); });
        
	let surface = Surface::load_bmp(&Path::new("data/test.bmp"))
		.unwrap_or_else(|e| {	prompt_error_and_panic("Couldn't load BMP", &e, None); });
	
	let mut running = true;
	while running 
  {
		for event in event_pump.poll_iter()  // equivalent of SDL_PollEvent in a loop
    {
			use sdl2::event::Event;
			match event 
      {
				Event::Quit {..} => { running = false },
        _ => {}
      }
    }
    
    // Ok to borrow the pump as long as we've finished the event handling
		let mut wsuf = window.surface(&event_pump).unwrap();
		surface.blit(None, &mut wsuf, None)?; 
	 
		wsuf.update_window()?;
	}
	
	Ok(())
}
