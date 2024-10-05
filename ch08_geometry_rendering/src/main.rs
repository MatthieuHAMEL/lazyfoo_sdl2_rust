extern crate sdl2; 

use sdl2::Sdl;
use sdl2::IntegerOrSdlError;
use sdl2::image::{Sdl2ImageContext, InitFlag};
use sdl2::VideoSubsystem;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::Canvas;

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
    
    // <=> SDL_SetRenderDrawColor, here inside the loop because it will change
    canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF)); // white
    canvas.clear();
    
    canvas.set_draw_color(Color::RGBA(0xFF, 0x00, 0x00, 0xFF)); // red
    canvas.fill_rect(Rect::new((WINDOW_WIDTH as i32) / 4, 
                               (WINDOW_HEIGHT as i32) / 4, 
                                WINDOW_WIDTH / 2, 
                                WINDOW_HEIGHT / 2))?;
    
    // fill_rect creates a rectangle filled with the draw color, draw_rect just creates the shape:
    
    canvas.set_draw_color(Color::RGBA(0x00, 0xFF, 0x00, 0xFF)); // green
    canvas.draw_rect(Rect::new((WINDOW_WIDTH as i32) / 6, 
                               (WINDOW_HEIGHT as i32) / 6, 
                               WINDOW_WIDTH * 2 / 3, 
                               WINDOW_HEIGHT * 2 / 3))?;
    
    canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0xFF, 0xFF)); // blue
    canvas.draw_line(Point::new(0, (WINDOW_HEIGHT as i32) / 2),
                     Point::new(WINDOW_WIDTH as i32, (WINDOW_HEIGHT as i32) / 2))?;
    
    canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0x00, 0xFF)); // blue
    for i in (0..WINDOW_HEIGHT).step_by(4) {
      canvas.draw_point(Point::new((WINDOW_WIDTH as i32)/2, i as i32))?;
    }
    
    // <=> SDL_RenderPresent
    canvas.present(); 
  }
	
  Ok(())
}
