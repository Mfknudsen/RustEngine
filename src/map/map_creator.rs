use rand::Rng;
use sdl2::pixels::Color;
use std::sync::{Arc, Mutex};

use crate::{
    characters::{flag::Flag, gumba::Gumba, gura::Gura, player::Player},
    map::map_collider::MapCollider,
    traits::npc::NPC,
    DrawBox, WINDOW_HEIGHT,
};

pub fn generate(
    player_name: String,
) -> (
    Vec<DrawBox>,
    Vec<DrawBox>,
    Vec<MapCollider>,
    Arc<Mutex<Player>>,
    Vec<Box<dyn NPC>>,
) {
    //
    // BACKGROUND
    //
    let static_map_background_boxes: Vec<DrawBox> = Vec::new();

    //
    // LEVEL
    //
    let mut static_map_boxes: Vec<DrawBox> = Vec::new();
    let mut static_map_colliders: Vec<MapCollider> = Vec::new();

    let mut r = generate_ground_with_collider(0.0, WINDOW_HEIGHT as f32 - 200.0, 2000.0, 600.0);
    static_map_boxes.append(&mut r.0);
    static_map_colliders.push(r.1);

    r = generate_ground_with_collider(2100.0, WINDOW_HEIGHT as f32 - 200.0, 2000.0, 600.0);
    static_map_boxes.append(&mut r.0);
    static_map_colliders.push(r.1);

    //static_map_boxes.push(DrawBox::new(0.0, WINDOW_HEIGHT as f32 - 200.0, 2000, 100, Color::GREEN));
    //static_map_boxes.push(DrawBox::new(0.0, WINDOW_HEIGHT as f32 - 100.0, 2000, 500, Color::RGB(108, 26, 26)));
    //static_map_colliders.push(MapCollider::new(0.0, WINDOW_HEIGHT as f32 - 200.0, 2000.0, 600.0));

    //
    // PLAYER
    //
    let player = Arc::new(Mutex::new(Player::new(50.0, 500.0, player_name)));
    // let mut player = Player::new(50.0, 500.0);

    //
    // GUMBAS
    //

    let mut npcs: Vec<Box<dyn NPC>> = Vec::new();

    match Gura::new(700.0, 500.0) {
        Some(gura) => npcs.push(Box::new(gura)),
        None => println!("Failed to create Gura"),
    }

    match Gumba::new(600.0, 500.0) {
        Ok(gumba) => npcs.push(Box::new(gumba)),
        Err(e) => println!("Failed to create Gumba: {}", e),
    }
    match Gumba::new(-300.0, 500.0) {
        Ok(gumba) => npcs.push(Box::new(gumba)),
        Err(e) => println!("Failed to create Gumba: {}", e),
    }

    match Flag::new(3800.0, 300.0) {
        Some(flag) => npcs.push(Box::new(flag)),
        None => println!("Failed to create Flag"),
    }

    return (
        static_map_background_boxes,
        static_map_boxes,
        static_map_colliders,
        player,
        npcs,
    );
}

fn generate_ground_with_collider(
    x: f32,
    y: f32,
    x_size: f32,
    y_size: f32,
) -> (Vec<DrawBox>, MapCollider) {
    let mut result: Vec<DrawBox> = Vec::new();
    let collider: MapCollider = MapCollider::new(x, y, x_size, y_size);

    result.push(DrawBox::new(
        x,
        y + y_size * 0.1,
        x_size as u32,
        (y_size * 0.9) as u32,
        Color::RGB(108, 26, 26),
    ));
    result.push(DrawBox::new(
        x,
        y,
        x_size as u32,
        (y_size * 0.1) as u32,
        Color::GREEN,
    ));

    let n = (x_size / 50.0) as u32;
    for _i in 0..n {
        let x_point = rand::thread_rng().gen_range(10.0..(x_size - 40.0)) + x;
        let y_point = rand::thread_rng().gen_range(10.0..y_size * 0.9 - 10.0) + y + y_size * 0.1;

        result.push(DrawBox::new(
            x_point,
            y_point,
            30,
            15,
            Color::RGB(138, 56, 56),
        ))
    }

    (result, collider)
}
