use std::sync::{Arc, Mutex};
use crate::characters::player::Player;
use crate::map::map_collider::MapCollider;

pub trait BoxCollider {
    fn move_x(&self) -> f32 {
        0.0
    }
    fn move_y(&self) -> f32 {
        0.0
    }
    fn x_position(&self) -> f32;
    fn y_position(&self) -> f32;

    fn set_x_position(&mut self, _set: f32) {}
    fn set_y_position(&mut self, _set: f32) {}

    fn x_size(&self) -> f32;
    fn y_size(&self) -> f32;

    fn set_x_velocity(&mut self, _set: f32) {}
    fn set_y_velocity(&mut self, _set: f32) {}

    fn set_grounded(&mut self, _set: bool) {}

    fn check_against_map(&mut self, map_colliders: &mut Vec<MapCollider>) {
        let move_x = self.move_x();
        let move_y = self.move_y();

        for col in &mut map_colliders.iter() {
            if move_y > 0.0 &&
                col.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32) &&
                col.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32) {
                self.set_y_position(col.y_position() - self.y_size());
                self.set_y_velocity(0.0);
                self.set_grounded(true);
            } else if move_y < 0.0 &&
                col.point_in_box(self.x_position() as i32, self.y_position() as i32) &&
                col.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32) {
                self.set_y_position(col.y_position() + col.y_size());
                self.set_y_velocity(0.0);
            }

            if move_x > 0.0 &&
                col.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32) &&
                col.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32) {
                self.set_x_position(col.x_position() - self.x_size());
                self.set_x_velocity(0.0);
            } else if move_x < 0.0 &&
                col.point_in_box(self.x_position() as i32, self.y_position() as i32) &&
                col.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32) {
                self.set_x_position(col.x_position() + col.x_size());
                self.set_x_velocity(0.0);
            }

            if move_x > 0.0 &&
                (col.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32) ||
                    col.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32)) {
                self.set_x_position(col.x_position() - self.x_size());
                self.set_x_velocity(0.0);
            } else if move_x < 0.0 &&
                (col.point_in_box(self.x_position() as i32, self.y_position() as i32) ||
                    col.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32)) {
                self.set_x_position(col.x_position() + col.x_size());
                self.set_x_velocity(0.0);
            }

            if move_y > 0.0 &&
                (col.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32) ||
                    col.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32)) {
                self.set_y_position(col.y_position() - self.y_size());
                self.set_y_velocity(0.0);
                self.set_grounded(true);
            } else if move_y < 0.0 &&
                (col.point_in_box(self.x_position() as i32, self.y_position() as i32) ||
                    col.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32)) {
                self.set_y_position(col.y_position() + col.y_size());
                self.set_y_velocity(0.0);
            }
        }
    }

    fn check_against_player(&mut self, a_player: &Arc<Mutex<Player>>) -> bool {
        let move_x = self.move_x();
        let move_y = self.move_y();

        let player = a_player.lock().unwrap();
        (move_y > 0.0 &&
            player.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32) &&
            player.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32)) ||
            (move_y < 0.0 &&
                player.point_in_box(self.x_position() as i32, self.y_position() as i32) &&
                player.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32)) ||
            (move_x > 0.0 &&
                player.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32) &&
                player.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32)) ||
            (move_x < 0.0 &&
                player.point_in_box(self.x_position() as i32, self.y_position() as i32) &&
                player.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32)) ||
            (move_x > 0.0 &&
                (player.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32) ||
                    player.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32))) ||
            (move_x < 0.0 &&
                (player.point_in_box(self.x_position() as i32, self.y_position() as i32) ||
                    player.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32))) ||
            (move_y > 0.0 &&
                (player.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32) ||
                    player.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32))) ||
            (move_y < 0.0 &&
                (player.point_in_box(self.x_position() as i32, self.y_position() as i32) ||
                    player.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32)))
    }


    fn point_in_box(&self, x: i32, y: i32) -> bool {
        x > self.x_position() as i32 &&
            y > self.y_position() as i32 &&
            x < (self.x_position() + self.x_size()) as i32 &&
            y < (self.y_position() + self.y_size()) as i32
    }
}