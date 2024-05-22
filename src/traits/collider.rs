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
        let player = a_player.lock().unwrap();

        //On of the colliders corners is within the collider of the player
        if player.point_in_box(self.x_position() as i32, (self.y_position() + self.y_size()) as i32) ||
            player.point_in_box(self.x_position() as i32, self.y_position() as i32) ||
            player.point_in_box((self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32) ||
            player.point_in_box((self.x_position() + self.x_size()) as i32, self.y_position() as i32)
        {
            return true;
        }

        //Top
        if self.line_intersect_line(self.x_position() as i32, self.y_position() as i32,
                                    (self.x_position() + self.x_size()) as i32, self.y_position() as i32,
                                    player.x_position() as i32, (player.y_position()) as i32,
                                    player.x_position() as i32, (player.y_position() + player.y_size()) as i32) {
            return true;
        }
        if self.line_intersect_line(self.x_position() as i32, self.y_position() as i32,
                                    (self.x_position() + self.x_size()) as i32, self.y_position() as i32,
                                    (player.x_position() + player.x_size()) as i32, (player.y_position()) as i32,
                                    (player.x_position() + player.x_size()) as i32, (player.y_position() + player.y_size()) as i32) {
            return true;
        }

        //Bop
        if self.line_intersect_line(self.x_position() as i32, (self.y_position() + player.y_size()) as i32,
                                    (self.x_position() + self.x_size()) as i32, (self.y_position() + player.y_size()) as i32,
                                    player.x_position() as i32, (player.y_position()) as i32,
                                    player.x_position() as i32, (player.y_position() + player.y_size()) as i32) {
            return true;
        }
        if self.line_intersect_line(self.x_position() as i32, (self.y_position() + player.y_size()) as i32,
                                    (self.x_position() + self.x_size()) as i32, (self.y_position() + player.y_size()) as i32,
                                    (player.x_position() + player.x_size()) as i32, (player.y_position()) as i32,
                                    (player.x_position() + player.x_size()) as i32, (player.y_position() + player.y_size()) as i32) {
            return true;
        }

        //Left
        if self.line_intersect_line(self.x_position() as i32, self.y_position() as i32,
                                    self.x_position() as i32, (self.y_position() + self.y_size()) as i32,
                                    player.x_position() as i32, (player.y_position()) as i32,
                                    (player.x_position() + player.x_size()) as i32, player.y_position() as i32) {
            return true;
        }
        if self.line_intersect_line(self.x_position() as i32, self.y_position() as i32,
                                    self.x_position() as i32, (self.y_position() + self.y_size()) as i32,
                                    player.x_position() as i32, (player.y_position() + player.y_size()) as i32,
                                    (player.x_position() + player.x_size()) as i32, (player.y_position() + player.y_size()) as i32) {
            return true;
        }

        //Right
        if self.line_intersect_line((self.x_position() + self.x_size()) as i32, self.y_position() as i32,
                                    (self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32,
                                    player.x_position() as i32, (player.y_position()) as i32,
                                    (player.x_position() + player.x_size()) as i32, player.y_position() as i32) {
            return true;
        }
        if self.line_intersect_line((self.x_position() + self.x_size()) as i32, self.y_position() as i32,
                                    (self.x_position() + self.x_size()) as i32, (self.y_position() + self.y_size()) as i32,
                                    player.x_position() as i32, (player.y_position() + player.y_size()) as i32,
                                    (player.x_position() + player.x_size()) as i32, (player.y_position() + player.y_size()) as i32) {
            return true;
        }


        false
    }


    fn point_in_box(&self, x: i32, y: i32) -> bool {
        x > self.x_position() as i32 &&
            y > self.y_position() as i32 &&
            x < (self.x_position() + self.x_size()) as i32 &&
            y < (self.y_position() + self.y_size()) as i32
    }

    //1 and 2 is one line, 3 and 4 is one line
    fn line_intersect_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> bool {
        if (x1 < x3 && x2 < x3) ||
            (x1 > x3 && x2 > x3) {
            return false;
        }

        if (y1 < y3 && y2 < y3) ||
            (y1 > y3 && y2 > y3) {
            return false;
        }

        //First line is horizontal
        if x1 == x2 &&
            ((x3 < x2 && x3 > x1) || (x3 > x2 && x3 < x1)) &&
            ((y3 < y1 && y4 > y1) || (y3 > y1 && y4 < y1)) {
            return false;
        }

        //First line is vertical
        if y1 == y2 &&
            ((x3 < x2 && x3 > x1) || (x3 > x2 && x3 < x1)) &&
            ((y3 < y1 && y4 > y1) || (y3 > y1 && y4 < y1)) {
            return false;
        }

        true
    }
}