use sdl2::pixels::Color;

use crate::{
    DrawBox,
    get_delta_time,
    traits::{
        character::Character,
        collider::BoxCollider,
        drawer::Drawer,
        npc::NPC,
        transform::Transform
    }
};

const GUMBA_MOVE_SPEED: f32 = 250.0;

pub struct Gura {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    box_x_size: f32,
    box_y_size: f32,
    boxes: Vec<DrawBox>,
    walk_direction: f32,
    dead: bool,
    state: State,
    state_timer: f32,
}

pub enum State{
    Idle,
    Move,
    Run,
}




impl Gura {
    pub(crate) fn new(x_start: f32, y_start: f32) -> Option<Self> {
        if x_start < 0.0 || y_start < 0.0 {
            None
        } else {
            let mut r = Self {
                x: x_start,
                y: y_start,
                x_velocity: 0.0,
                y_velocity: 0.0,
                box_x_size: 50.0,
                box_y_size: 50.0,
                boxes: Vec::new(),
                walk_direction: -1.0,
                dead: false,
                state: State::Idle,
                state_timer: 0.0,
            };
            r.boxes = r.setup_boxes();

            Some(r)
        }
    }
}

impl Transform for Gura {
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

}

impl Drawer for Gura {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_boxes(&self) -> &Vec<DrawBox> {
        &self.boxes
    }

    fn setup_boxes(&self) -> Vec<DrawBox> {
        let mut result = Vec::new();

        result.push(DrawBox::new(0.0, 0.0, 50, 50, Color::CYAN));

        result
    }
}

impl BoxCollider for Gura {
    fn move_x(&self) -> f32 {
        self.x_velocity + self.walk_direction * GUMBA_MOVE_SPEED * get_delta_time()
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
}

impl NPC for Gura {}

impl Character for Gura {
    fn update(&mut self) {
        //Gravity
        self.y += self.y_velocity * get_delta_time();

        //state machine

        //This part is so that the npc can change between states
        self.state_timer += get_delta_time();
        if self.state_timer > 2.0 {
            // Reset the state timer
            self.state_timer = 0.0;
            // Change the state
            match self.state {
                State::Idle => {
                    self.state = State::Move;
                }
                State::Move => {
                    self.state = State::Run;
                }
                State::Run => {
                    self.state = State::Idle;
                }
            }
        }
        //Actual state machine stuff
        match self.state {
            State::Idle => {
                return;
            }
            State::Move => {
                self.x += self.walk_direction * GUMBA_MOVE_SPEED * get_delta_time();
                self.x += self.x_velocity * get_delta_time();
            }
            State::Run => {
                self.x += self.walk_direction * GUMBA_MOVE_SPEED * get_delta_time() * 2.0;
                self.x += self.x_velocity * get_delta_time();
            }
        }
    }

    fn should_remove(&self) -> bool {
        self.dead
    }

}


