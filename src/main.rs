use std::fmt::Debug;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::characters::player::Player;
use crate::traits::collider::BoxCollider;
use crate::traits::drawer::Drawer;
use crate::traits::transform::Transform;

mod map;
mod characters;
mod traits;


const WINDOW_WIDTH: u32 = 640 * 2;
const WINDOW_HEIGHT: u32 = 480 * 2;

const GRAVITY: f32 = 100.0;

static mut GLOBAL_PLAYER_X_OFFSET: f32 = 0.0;
static mut GLOBAL_PLAYER_Y_OFFSET: f32 = 0.0;

static mut DELTA_TIME: f32 = 0.0;

static mut PREVIOUS_TIME: f32 = 0.0;

//Main Function
fn main() -> Result<(), String> { //Returns OK(()) or error message in string form
    let sdl_context = sdl2::init()?; //Initializes the SDL2 Library for usage. "?" is
    //for error propagation
    let video_subsystem = sdl_context.video()?; //Gets video subsystem from SDL2 and
    //returns an object. "?" is for
    //error propagation
    let window = video_subsystem //Creates a window object using variables
        .window("Rust Exam | Mario Game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    //Generates a mutable canvas object
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    //Setting parameters for canvas
    canvas.set_draw_color(Color::RGB(255, 140, 0)); //Background Color
    canvas.clear(); //Clearing canvas from previous activity
    canvas.present(); //Updates canvas to show recent activity

    let mut event_pump = sdl_context.event_pump()?; //Responsible for collecting and
    //dispatching events

    let generator_result = map::map_creator::generate();
    let mut static_map_boxes = generator_result.0;
    let mut static_map_colliders = generator_result.1;
    let mut player = generator_result.2;
    let mut gumbas = generator_result.3;

    'running: loop { //Creates loop with "running" label to break out of later
        for event in event_pump.poll_iter() {
            match event { //Matches given pattern or keypress to event
                Event::Quit { .. } //Quit event for closing window
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), //If escape key is pressed...
                    ..
                } => break 'running, //...Break out of running loop

                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    player.update_input(Keycode::A, true);
                    break;
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    player.update_input(Keycode::D, true);
                    break;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    player.update_input(Keycode::Space, true);
                    break;
                }

                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    player.update_input(Keycode::A, false);
                    break;
                }
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    player.update_input(Keycode::D, false);
                    break;
                }

                _ => {}
            }
        }

        for mut g in &mut gumbas {
            g.add_force(0.0, GRAVITY * get_delta_time())
        }
        player.add_force(0.0, GRAVITY * get_delta_time());
        player.check_against_map(&mut static_map_colliders);

        update_delta_time();
        update_global_player_offset(&player);

        //Clearing canvas after one full movement (erases previous moves and objects)
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for box_obj in &mut static_map_boxes { //Iterates over each box, allows for
            box_obj.draw(&mut canvas)?; //Draws new box with changes in
        }

        player.update();
        player.draw_on_canvas(&mut canvas);

        canvas.present(); //Updates canvas
    }

    Ok(()) //Return OK / End of program
}

fn update_delta_time() {
    unsafe {
        let now = Instant::now();

        DELTA_TIME = (now.elapsed().as_secs_f32() - PREVIOUS_TIME) * 1000.0;

        if DELTA_TIME < 0.0 {
            DELTA_TIME *= -1.0;
        }

        PREVIOUS_TIME = now.elapsed().as_secs_f32();
    }
}

fn update_global_player_offset(player: &Player) {
    let lerp: f32 = 0.9;

    unsafe {
        let mut x = GLOBAL_PLAYER_X_OFFSET - player.get_x();
        GLOBAL_PLAYER_X_OFFSET -= x * lerp * get_delta_time();

        if GLOBAL_PLAYER_X_OFFSET < (WINDOW_WIDTH / 2) as f32 {
            GLOBAL_PLAYER_X_OFFSET = (WINDOW_WIDTH / 2) as f32;
        }

        let mut y = GLOBAL_PLAYER_Y_OFFSET - player.get_y();
        GLOBAL_PLAYER_Y_OFFSET -= y * lerp * get_delta_time();

        if GLOBAL_PLAYER_Y_OFFSET > ((WINDOW_HEIGHT / 2) as f32) - 400.0{
            GLOBAL_PLAYER_Y_OFFSET = ((WINDOW_HEIGHT / 2) as f32) - 400.0
        }
    }
}

fn get_delta_time() -> f32 {
    unsafe { return DELTA_TIME; }
}

fn get_global_player_x_offset() -> f32 {
    unsafe { return GLOBAL_PLAYER_X_OFFSET; }
}

fn get_global_player_y_offset() -> f32 {
    unsafe { return GLOBAL_PLAYER_Y_OFFSET; }
}

pub(crate) struct DrawBox {
    //Construct for box
    x_offset: f32,
    //x position in window
    y_offset: f32,
    //y position in window
    box_width: u32,
    //Width of box
    box_height: u32,
    //height of box
    box_color: Color,
    //color of box
}

impl DrawBox {
    //Implementation of box (all box related functions)
    fn new( //Create new box object
            x: f32,
            y: f32,
            box_width: u32,
            box_height: u32,
            box_color: Color,
    ) -> Self {
        Self {
            x_offset: x,
            y_offset: y,
            box_width,
            box_height,
            box_color,
        }
    }

    //Draws filled rectangle with specified position, width, height, and color
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let rect = Rect::new((self.x_offset - get_global_player_x_offset()) as i32, (self.y_offset - get_global_player_y_offset()) as i32, self.box_width, self.box_height);
        canvas.set_draw_color(self.box_color);
        canvas.fill_rect(rect)
    }
}