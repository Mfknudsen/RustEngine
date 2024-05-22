use sdl2::{keyboard::Keycode, pixels::Color};

use crate::{
    get_delta_time,
    traits::{character::Character, collider::BoxCollider, drawer::Drawer, transform::Transform},
    DrawBox,
};

const PLAYER_MOVE_SPEED: f32 = 1750.0;

pub struct Player {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    box_x_size: f32,
    box_y_size: f32,
    boxes: Vec<DrawBox>,
    keyboard_a: bool,
    keyboard_d: bool,
    grounded: bool,
    name: String,
}

impl Player {
    pub(crate) fn new(x_start: f32, y_start: f32, name: String) -> Self {
        let mut r =  Self {
            x: x_start,
            y: y_start,
            x_velocity: 0.0,
            y_velocity: 0.0,
            box_x_size: 50.0,
            box_y_size: 100.0,
            keyboard_d: false,
            keyboard_a: false,
            grounded: false,
            boxes: Vec::new(),
            name,
        };
        r.boxes = r.setup_boxes();
        r
    }

    pub(crate) fn update_input(&mut self, key_code: Keycode, key_down: bool) {
        if key_code == Keycode::A {
            self.keyboard_a = key_down;
        }
        if key_code == Keycode::D {
            self.keyboard_d = key_down;
        }

        if key_code == Keycode::Space && key_down && self.grounded {
            self.y_velocity = 0.0;
            self.add_force(0.0, -2500.0);
            self.grounded = false;
        }
    }

    pub fn get_name (&self) -> &String {
        &self.name
    }
}

impl Transform for Player {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, set: f32) {
        self.x = set;
    }

    fn set_y(&mut self, set: f32) {
        self.y = set;
    }

    fn get_x_velocity(&self) -> f32 {
        self.x_velocity
    }

    fn get_y_velocity(&self) -> f32 {
        self.y_velocity
    }

    fn set_x_velocity(&mut self, set: f32) {
        self.x_velocity = set;
    }

    fn set_y_velocity(&mut self, set: f32) {
        self.y_velocity = set;
    }

    fn add_force(&mut self, x: f32, y: f32) {
        self.x_velocity += x;
        self.y_velocity += y;
    }
}

impl Drawer for Player {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_boxes(&self) -> &Vec<DrawBox> {
        self.boxes.as_ref()
    }

    fn setup_boxes(&self) -> Vec<DrawBox> {
        let mut result: Vec<DrawBox> = Vec::new();

        result.push(DrawBox::new(0.0, 0.0, 50, 100, Color::BLUE));

        result
    }
}

impl BoxCollider for Player {
    fn move_x(&self) -> f32 {
        let mut move_x: f32 = 0.0;
        if self.keyboard_a {
            move_x += -1.0;
        }
        if self.keyboard_d {
            move_x += 1.0;
        }
        self.x_velocity + move_x * PLAYER_MOVE_SPEED * get_delta_time()
    }

    fn move_y(&self) -> f32 {
        self.y_velocity
    }

    fn x_position(&self) -> f32 {
        self.x
    }

    fn y_position(&self) -> f32 {
        self.y
    }

    fn set_x_position(&mut self, set: f32) {
        self.x = set;
    }

    fn set_y_position(&mut self, set: f32) {
        self.y = set;
    }

    fn x_size(&self) -> f32 {
        self.box_x_size
    }

    fn y_size(&self) -> f32 {
        self.box_y_size
    }

    fn set_x_velocity(&mut self, set: f32) {
        self.x_velocity = set;
    }

    fn set_y_velocity(&mut self, set: f32) {
        self.y_velocity = set;
    }

    fn set_grounded(&mut self, set: bool) {
        self.grounded = set;
    }
}

impl Character for Player {
    fn update(&mut self) {
        let mut move_x: f32 = 0.0;
        if self.keyboard_a {
            move_x += -1.0;
        }
        if self.keyboard_d {
            move_x += 1.0;
        }
        self.x += move_x * PLAYER_MOVE_SPEED * get_delta_time();

        self.x += self.x_velocity * get_delta_time();
        self.y += self.y_velocity * get_delta_time();
    }

    fn should_remove(&self) -> bool {
        false
    }
}


