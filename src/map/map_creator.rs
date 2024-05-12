use sdl2::pixels::Color;

use crate::characters::{player::Player, gumba::Gumba};
use crate::DrawBox;
use crate::map::map_collider::MapCollider;
use crate::WINDOW_HEIGHT;

pub fn generate() -> (Vec<DrawBox>, Vec<MapCollider>, Player, Vec<Gumba>) {
    let mut static_map_boxes = Vec::new();
    let mut static_map_colliders = Vec::new();
    let mut turtles = Vec::new();

    static_map_boxes.push(DrawBox::new(0.0, WINDOW_HEIGHT as f32 - 100.0, 2000, 500, Color::RGB(108, 26, 26)));
    static_map_boxes.push(DrawBox::new(0.0, WINDOW_HEIGHT as f32 - 200.0, 2000, 100, Color::GREEN));

    static_map_colliders.push(MapCollider::new(0.0, WINDOW_HEIGHT as f32 - 100.0, 2000.0, 200.0));

    let mut player = Player::new(50.0, 50.0);
    turtles.push(Gumba::new(250.0, 50.0));
    turtles.push(Gumba::new(350.0, 50.0));

    return (static_map_boxes, static_map_colliders, player, turtles);
}