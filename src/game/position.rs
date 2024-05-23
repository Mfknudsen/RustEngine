use std::{error::Error, time::Instant};

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas };

use crate::{
    characters::player::Player,
    traits::{collider::BoxCollider, npc::NPC, transform::Transform},
};

const GRAVITY: f32 = 9810.0;

static mut GLOBAL_PLAYER_X_OFFSET: f32 = 0.0;
static mut GLOBAL_PLAYER_Y_OFFSET: f32 = 0.0;

static mut DELTA_TIME: f32 = 0.0;

static mut PREVIOUS_TIME: f32 = 0.0;

pub fn update_global_offsets(window_width: u32, window_height: u32, player_lock: &Player) {
    let half_x = (window_width / 2) as f32;
    let half_y = (window_height / 2) as f32;
    
    unsafe {
        GLOBAL_PLAYER_X_OFFSET = -player_lock.x_position() + player_lock.x_size() / 2.0 + half_x;
        GLOBAL_PLAYER_Y_OFFSET = -player_lock.y_position() / 2.0 + half_y;
    }
}

pub fn update_canvas(player_lock: &Player, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = font_context.load_font("src/extra/HackNerdFont-Regular.ttf", 24)?;

    let surface = font
        .render(player_lock.get_name())
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;

    let temp = canvas.texture_creator();
    let texture = surface.as_texture(&temp)?;

    let target = Rect::new(
        (player_lock.x_position() - player_lock.x_size()
            + get_global_player_x_offset()
            + 35.0) as i32,
        (player_lock.y_position() - player_lock.y_size() + get_global_player_y_offset()
            - 15.0) as i32,
        surface.width(),
        surface.height(),
    );

    canvas.copy(&texture, None, target)?;

    canvas.present();
    Ok(())
}

pub fn update_delta_time() {
    unsafe {
        let now = Instant::now();

        DELTA_TIME = (now.elapsed().as_secs_f32() - PREVIOUS_TIME) * 1000.0;

        if DELTA_TIME < 0.0 {
            DELTA_TIME *= -1.0;
        }

        PREVIOUS_TIME = now.elapsed().as_secs_f32();
    }
}

pub fn update_global_player_offset(window_width: u32, window_height: u32, player: &Player) {
    let lerp: f32 = 0.8;
    let speed: f32 = 50.0;

    let half_x: f32 = (window_width / 2) as f32;
    let half_y: f32 = (window_height / 2) as f32;

    unsafe {
        let towards_target_x: f32 =
            (-Transform::get_x(player) + half_x - 50.0) - GLOBAL_PLAYER_X_OFFSET;
        GLOBAL_PLAYER_X_OFFSET += towards_target_x * lerp * speed * get_delta_time();

        if GLOBAL_PLAYER_X_OFFSET > ((window_width / 2) as f32) - 500.0 {
            GLOBAL_PLAYER_X_OFFSET = ((window_width / 2) as f32) - 500.0;
        }

        let towards_target_y: f32 =
            (-Transform::get_y(player) + player.y_size() / 2.0 + half_y) - GLOBAL_PLAYER_Y_OFFSET;
        GLOBAL_PLAYER_Y_OFFSET += towards_target_y * lerp * speed * get_delta_time();

        if GLOBAL_PLAYER_Y_OFFSET < ((window_height / 2) as f32) - 550.0 {
            GLOBAL_PLAYER_Y_OFFSET = ((window_height / 2) as f32) - 550.0
        }
    }
}

pub fn apply_gravity(player: &mut Player, npcs: &mut Vec<Box<dyn NPC>>) -> () {
    player.add_force(0.0, GRAVITY * get_delta_time());

for g in npcs {
    g.add_force(0.0, GRAVITY * get_delta_time())
}
}

pub fn get_delta_time() -> f32 {
    unsafe {
        return DELTA_TIME;
    }
}

pub fn get_global_player_x_offset() -> f32 {
    unsafe {
        return GLOBAL_PLAYER_X_OFFSET;
    }
}

pub fn get_global_player_y_offset() -> f32 {
    unsafe {
        return GLOBAL_PLAYER_Y_OFFSET;
    }
}