use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::{Window, WindowBuilder};
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::EventPump;
use sdl2::sys::SDL_WindowFlags;

pub fn init_sdl(window_width: u32, window_height: u32) -> Result<(WindowCanvas, EventPump), String> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;

    let mut builder: WindowBuilder = video_subsystem.window("Rust Exam | Mario Game", window_width, window_height);
    builder.set_window_flags(SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32);

    let window: Window = builder.position_centered().build().map_err(|e| e.to_string())?;

    let mut canvas: WindowCanvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(255, 140, 0)); // Background Color
    canvas.clear(); // Clearing canvas from previous activity
    canvas.present(); // Updates canvas to show recent activity

    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}