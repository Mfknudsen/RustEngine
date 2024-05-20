use sdl2::pixels::Color;
use rand::Rng;

use crate::{
    characters::{gumba::Gumba,gura::Gura, player::Player},
    traits::npc::NPC,
    DrawBox,
    map::map_collider::MapCollider,
    name::get_name_input,
    WINDOW_HEIGHT,
};


pub fn generate(player_name: String) -> (Vec<DrawBox>, Vec<DrawBox>, Vec<MapCollider>, Player, Vec<Box<dyn NPC>>) {
    ///
    /// BACKGROUND
    ///
    let mut static_map_background_boxes: Vec<DrawBox> = Vec::new();

    ///
    /// LEVEL
    ///
    let mut static_map_boxes: Vec<DrawBox> = Vec::new();
    let mut static_map_colliders: Vec<MapCollider> = Vec::new();

    let mut r = generate_ground_with_collider(0.0, WINDOW_HEIGHT as f32 - 200.0, 2000.0, 600.0);
    static_map_boxes.append(&mut r.0);
    static_map_colliders.push(r.1);

    r = generate_ground_with_collider(2100.0, WINDOW_HEIGHT as f32 - 200.0, 2000.0, 600.0);
    static_map_boxes.append(&mut r.0);
    static_map_colliders.push(r.1);

    ///static_map_boxes.push(DrawBox::new(0.0, WINDOW_HEIGHT as f32 - 200.0, 2000, 100, Color::GREEN));
    ///static_map_boxes.push(DrawBox::new(0.0, WINDOW_HEIGHT as f32 - 100.0, 2000, 500, Color::RGB(108, 26, 26)));
    ///static_map_colliders.push(MapCollider::new(0.0, WINDOW_HEIGHT as f32 - 200.0, 2000.0, 600.0));

    ///
    /// PLAYER
    ///
    let mut player = Player::new(50.0, 500.0, player_name);

    ///
    /// GUMBAS
    ///


    let mut turtles: Vec<Box<dyn NPC>> = Vec::new();

    turtles.push(Box::new(Gura::new(200.0, 500.0)));
    turtles.push(Box::new(Gumba::new(300.0, 500.0)));
    turtles.push(Box::new(Gumba::new(440.0, 500.0)));
    turtles.push(Box::new(Gumba::new(360.0, 500.0)));

    return (static_map_background_boxes, static_map_boxes, static_map_colliders, player, turtles);
}

fn generate_ground_with_collider(x: f32, y: f32, x_size: f32, y_size: f32) -> (Vec<DrawBox>, MapCollider) {
    let mut result: Vec<DrawBox> = Vec::new();
    let mut collider: MapCollider = MapCollider::new(x, y, x_size, y_size);

    result.push(DrawBox::new(x, y + y_size * 0.1, x_size as u32, (y_size * 0.9) as u32, Color::RGB(108, 26, 26)));
    result.push(DrawBox::new(x, y, x_size as u32, (y_size * 0.1) as u32, Color::GREEN));

    let n = (x_size / 50.0) as u32;
    for _i in 0..n {
        let x_point = rand::thread_rng().gen_range(10.0..(x_size - 40.0)) + x;
        let y_point = rand::thread_rng().gen_range(10.0..y_size * 0.9 - 10.0) + y + y_size * 0.1;

        result.push(DrawBox::new(x_point, y_point, 30, 15, Color::RGB(138, 56, 56)))
    }

    (result, collider)
}