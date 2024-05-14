use crate::map::map_collider::MapCollider;

pub trait BoxCollider {
    fn x_position(&self) -> f32;
    fn y_position(&self) -> f32;

    fn x_size(&self) -> f32;
    fn y_size(&self) -> f32;

    fn check_against_map(&mut self, map_colliders: &mut Vec<MapCollider>);

    fn point_in_box(&self, x: i32, y: i32) -> bool;
}