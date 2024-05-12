use crate::map::map_collider::MapCollider;

pub trait BoxCollider {
    fn x_position(&self) -> f32;
    fn y_position(&self) -> f32;

    fn x_size(&self) -> f32;
    fn y_size(&self) -> f32;

    fn x_center(&self) -> f32;
    fn y_center(&self) -> f32;

    fn check_against_map(&self, map_colliders: &mut Vec<MapCollider>);
}