use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, keyboard::Keycode};

use crate::{DrawBox, get_delta_time};
use crate::map::map_collider::MapCollider;
use crate::traits::{drawer::Drawer, transform::Transform, collider::BoxCollider};

const PLAYER_MOVE_SPEED: f32 = 1000.0;

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
}

impl Player {
    pub(crate) fn new(x_start: f32, y_start: f32) -> Self {
        Self {
            x: x_start,
            y: y_start,
            x_velocity: 0.0,
            y_velocity: 0.0,
            box_x_size: 50.0,
            box_y_size: 100.0,
            keyboard_d: false,
            keyboard_a: false,
            grounded: false,
            boxes: Self::setup_boxes(),
        }
    }

    fn setup_boxes() -> Vec<DrawBox> {
        let mut result: Vec<DrawBox> = Vec::new();

        result.push(DrawBox::new(0.0, 0.0, 50, 100, Color::BLUE));

        result
    }

    pub(crate) fn update_input(&mut self, key_code: Keycode, key_down: bool) {
        if key_code == Keycode::A
        {
            self.keyboard_a = key_down;
        }
        if key_code == Keycode::D
        {
            self.keyboard_d = key_down;
        }

        if key_code == Keycode::Space && key_down {
            self.add_force(0.0, -1000.0);
            self.grounded = false;
        }
    }

    pub(crate) fn update(&mut self) {
        let mut move_x: f32 = 0.0;
        if self.keyboard_a {
            move_x += -1.0;
        }
        if self.keyboard_d {
            move_x += 1.0;
        }
        self.x += move_x * PLAYER_MOVE_SPEED * get_delta_time();

        if !self.grounded {
            self.y_velocity += 9.81 * get_delta_time();
        }

        self.x += self.x_velocity * get_delta_time();
        self.y += self.y_velocity * get_delta_time();
    }
}

impl Transform for Player {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn add_force(&mut self, x: f32, y: f32) {
        self.x_velocity += x;
        self.y_velocity += y;
    }
}

impl Transform for &mut Player {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn add_force(&mut self, x: f32, y: f32) {
        self.x_velocity += x;
        self.y_velocity += y;
    }
}

impl Drawer for Player {
    fn draw_on_canvas(&mut self, canvas: &mut WindowCanvas) {
        let x = self.get_x();
        let y = self.get_y();
        for box_obj in &mut self.boxes {
            let rect = Rect::new((box_obj.x_offset + x) as i32, (box_obj.y_offset + y) as i32, box_obj.box_width, box_obj.box_height);
            canvas.set_draw_color(box_obj.box_color);
            canvas.fill_rect(rect).expect("ERROR");
        }
    }
}

impl BoxCollider for Player {
    fn x_position(&self) -> f32 {
        self.x
    }

    fn y_position(&self) -> f32 {
        self.y
    }

    fn x_size(&self) -> f32 {
        self.box_x_size
    }

    fn y_size(&self) -> f32 {
        self.box_y_size
    }

    fn x_center(&self) -> f32 {
        self.x + self.box_x_size / 2.0
    }

    fn y_center(&self) -> f32 {
        self.y + self.box_y_size / 2.0
    }

    fn check_against_map(&self, map_colliders: &mut Vec<MapCollider>) {
        for col in &mut map_colliders.iter(){

        }
    }
}