use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use std::io::{self, Write};

use sdl2::libc::printf;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas,
    video::Window, Sdl, VideoSubsystem,
};

use crate::{
    characters::player::Player,
    traits::character::Character,
    traits::{collider::BoxCollider, drawer::Drawer, transform::Transform},
};

mod characters;
mod map;
mod traits;

const WINDOW_WIDTH: u32 = 640 * 2;
const WINDOW_HEIGHT: u32 = 480 * 2;

const GRAVITY: f32 = 9810.0;

static mut GLOBAL_PLAYER_X_OFFSET: f32 = 0.0;
static mut GLOBAL_PLAYER_Y_OFFSET: f32 = 0.0;

static mut DELTA_TIME: f32 = 0.0;

static mut PREVIOUS_TIME: f32 = 0.0;

fn main() -> Result<(), String> {
    fn get_name_input() -> String {
        loop {
            print!("Write your name: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut name_input = String::new();
            io::stdin()
                .read_line(&mut name_input)
                .expect("Failed to read line");
            return name_input;
        }
    }

    fn handle_string(name_input: String) -> String {
        let trimmed_name = name_input.trim();

        if !trimmed_name.is_empty() {
            let first_char = &trimmed_name[0..1].to_uppercase();
            let remaining_chars = &trimmed_name[1..];
            let modified_name = format!("{}{}", first_char, remaining_chars);
            modified_name
        } else {
            String::from("Mario")
        }
    }

    let name_input = get_name_input();
    let player_name = handle_string(name_input);

    println!("Your name is: {}", player_name);

    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;

    let window: Window = video_subsystem
        .window("Rust Exam | Mario Game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    //
    // Creating the canvas
    //
    let mut canvas: WindowCanvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(255, 140, 0)); //Background Color
    canvas.clear(); //Clearing canvas from previous activity
    canvas.present(); //Updates canvas to show recent activity

    let mut event_pump = sdl_context.event_pump()?; //Responsible for collecting and

    //
    // Generating the level
    // Includes background, interactable map and characters
    //
    let generator_result = map::map_creator::generate();
    let mut static_map_background_boxes = generator_result.0;
    let mut static_map_boxes = generator_result.1;
    let mut static_map_colliders = generator_result.2;
    let mut player = generator_result.3;
    let mut gumbas = generator_result.4;

    //
    // Start values for globals
    //
    unsafe {
        let half_x = (WINDOW_WIDTH / 2) as f32;
        let half_y = (WINDOW_HEIGHT / 2) as f32;

        GLOBAL_PLAYER_X_OFFSET = -player.get_x() + player.x_size() / 2.0 + half_x;
        GLOBAL_PLAYER_Y_OFFSET = -player.get_y() + player.y_size() / 2.0 + half_y;
    }

    //
    // Game loop
    //
    'game: loop {
        //Creates loop with "running" label to break out of later
        //
        // Reading player input events
        //
        for event in event_pump.poll_iter() {
            match event { //Matches given pattern or keypress to event
                Event::Quit { .. } //Quit event for closing window
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), //If escape key is pressed...
                    ..
                } => break 'game, //...Break out of running loop

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

        if !player_name.is_empty() {
            let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
            let font = font_context.load_font("src/extra/HackNerdFont-Regular.ttf", 24)?;

            let surface = font
                .render(&player_name)
                .blended(Color::WHITE)
                .map_err(|e| e.to_string())?;

            let temp = canvas.texture_creator();
            let texture_result = surface.as_texture(&temp);

            let texture = match texture_result {
                Ok(tex) => tex,

                Err(err) => {
                    // Handle the error case
                    eprintln!("Error converting surface to texture: {}", err);
                    return Err(err.to_string());
                }
            };

            let target = Rect::new(
                (player.get_x() - player.x_size() + get_global_player_x_offset() + 35.0) as i32,
                (player.get_y() - player.y_size() + get_global_player_y_offset() - 15.0) as i32,
                surface.width(),
                surface.height(),
            );
            canvas.copy(&texture, None, target)?;

            canvas.present();
        }

        //
        // Update globals
        //
        update_delta_time();
        update_global_player_offset(&player);

        //
        // Gravity force on all characters
        //
        player.add_force(0.0, GRAVITY * get_delta_time());
        for mut g in &mut gumbas {
            g.add_force(0.0, GRAVITY * get_delta_time())
        }

        //
        // Updating characters
        //
        player.update();

        ///
        /// Remove defeated
        ///
        let gumbalen = gumbas.len() - 1;
        for i in 0..(gumbalen) {
            if gumbas[gumbalen - i].should_remove() {
                gumbas.remove(gumbalen - i);
            }
        }

        //
        // Continue updating characters
        //
        for mut g in &mut gumbas {
            g.update();
        }

        //
        // Check collisions
        //
        player.check_against_map(&mut static_map_colliders);
        for mut g in &mut gumbas {
            g.check_against_map(&mut static_map_colliders)
        }

        //
        // Drawing the canvas
        //
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for box_obj in &mut static_map_background_boxes.iter() {
            box_obj.draw_no_offset(&mut canvas)?;
        }

        for box_obj in &mut static_map_boxes.iter() {
            //Iterates over each box, allows for
            box_obj.draw(0.0, 0.0, &mut canvas)?; //Draws new box with changes in
        }

        for mut g in &mut gumbas {
            g.draw_on_canvas(&mut canvas);
        }

        player.draw_on_canvas(&mut canvas);

        std::thread::sleep(Duration::from_millis(1));
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
    let speed: f32 = 100.0;

    let half_x: f32 = (WINDOW_WIDTH / 2) as f32;
    let half_y: f32 = (WINDOW_HEIGHT / 2) as f32;

    unsafe {
        let mut towards_target_x: f32 =
            (-player.get_x() + player.x_size() / 2.0 + half_x) - GLOBAL_PLAYER_X_OFFSET;
        GLOBAL_PLAYER_X_OFFSET += towards_target_x * lerp * speed * get_delta_time();

        if GLOBAL_PLAYER_X_OFFSET > ((WINDOW_WIDTH / 2) as f32) - 500.0 {
            GLOBAL_PLAYER_X_OFFSET = ((WINDOW_WIDTH / 2) as f32) - 500.0;
        }

        let mut towards_target_y: f32 =
            (-player.get_y() + player.y_size() / 2.0 + half_y) - GLOBAL_PLAYER_Y_OFFSET;
        GLOBAL_PLAYER_Y_OFFSET += towards_target_y * lerp * speed * get_delta_time();

        if GLOBAL_PLAYER_Y_OFFSET < ((WINDOW_HEIGHT / 2) as f32) - 550.0 {
            GLOBAL_PLAYER_Y_OFFSET = ((WINDOW_HEIGHT / 2) as f32) - 550.0
        }
    }
}

fn get_delta_time() -> f32 {
    unsafe {
        return DELTA_TIME;
    }
}

fn get_global_player_x_offset() -> f32 {
    unsafe {
        return GLOBAL_PLAYER_X_OFFSET;
    }
}

fn get_global_player_y_offset() -> f32 {
    unsafe {
        return GLOBAL_PLAYER_Y_OFFSET;
    }
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
    fn new(
        //Create new box object
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
    fn draw(
        &self,
        x: f32,
        y: f32,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        let rect = Rect::new(
            (x + self.x_offset + get_global_player_x_offset()) as i32,
            (y + self.y_offset + get_global_player_y_offset()) as i32,
            self.box_width,
            self.box_height,
        );
        canvas.set_draw_color(self.box_color);
        canvas.fill_rect(rect)
    }

    fn draw_no_offset(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        let rect = Rect::new(
            self.x_offset as i32,
            self.y_offset as i32,
            self.box_width,
            self.box_height,
        );
        canvas.set_draw_color(self.box_color);
        canvas.fill_rect(rect)
    }
}
