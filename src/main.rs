use std::{
    sync::{mpsc, Arc},
    thread,
    error::Error
};

use sdl2::pixels::Color;

use crate::{
    functions::name,
    traits::{
        collider::BoxCollider,
        drawer::Drawer,
        character::Character,
    },
    game::{setup, draw_box::DrawBox, position, events}
};

mod characters;
mod functions;
mod map;
mod traits;
mod game;

const WINDOW_WIDTH: u32 = 640 * 2;
const WINDOW_HEIGHT: u32 = 480 * 2;

enum ControlMessage {
    Break,
    // Other message types...
}

fn main() -> Result<(), Box<dyn Error>> {
    let name_input = name::get_name_input()?;
    
    let (mut canvas, mut event_pump) = setup::init_sdl(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    //
    // Generating the level
    // Includes background, interactable map and characters
    //
    let generator_result = map::map_creator::generate(name_input);
    let static_map_background_boxes = generator_result.0;
    let static_map_boxes = generator_result.1;
    let mut static_map_colliders = generator_result.2;
    let player = generator_result.3;
    let mut npcs = generator_result.4;

    //
    // Start values for globals
    //

    position::update_global_offsets(WINDOW_WIDTH, WINDOW_HEIGHT, &player.lock().unwrap());

    let (tx, rx) = mpsc::channel();

    //
    // Game loop
    //
    'game: loop {
        let mut handles = Vec::new();
        //Creates loop with "running" label to break out of later
        //
        // Reading player input events
        //
        for event in event_pump.poll_iter() {
            let player_clone = Arc::clone(&player);
            let tx = tx.clone();
            let handle = thread::spawn(move || {
                let player_lock = player_clone.lock().unwrap();
                if let Err(e) = events::handle_event(&tx, &event, player_lock) {
                    eprintln!("{}", e);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        if let Ok(ControlMessage::Break) = rx.try_recv() {
            break 'game;
        }

        if !player.lock().unwrap().get_name().is_empty() {
            position::update_canvas(&player.lock().unwrap(), &mut canvas)?;
        }

        //
        // Update globals
        //
        position::update_delta_time();
        position::update_global_player_offset(WINDOW_WIDTH, WINDOW_HEIGHT, &player.lock().unwrap()); //Uses dereferencing

        //
        // Gravity force on all characters
        //
        position::apply_gravity(&mut player.lock().unwrap(), &mut npcs);

        //
        // Updating characters
        //
        player.lock().unwrap().update();

        //
        // Remove defeated
        //
        let gumbalen = npcs.len() - 1;
        for i in 0..(gumbalen) {
            if npcs[gumbalen - i].should_remove() {
                npcs.remove(gumbalen - i);
            }
        }

        //
        // Continue updating characters
        //
        for g in &mut npcs {
            g.update();
        }

        //
        // Check collisions
        //
        player
            .lock()
            .unwrap()
            .check_against_map(&mut static_map_colliders);

        for g in &mut npcs {
            g.check_against_map(&mut static_map_colliders);
            if g.check_against_player(&player) {
                break 'game;
            };
        }

        //
        // Drawing the canvas
        //
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for box_obj in &mut static_map_background_boxes.iter() {
            box_obj.draw_no_offset(&mut canvas)?;
        }

        for box_obj in &mut static_map_boxes.iter() {
            //Iterates over each box, allows for
            box_obj.draw(0.0, 0.0, &mut canvas)?; //Draws new box with changes in
        }

        for g in &mut npcs {
            g.draw_on_canvas(&mut canvas);
        }

        player.lock().unwrap().draw_on_canvas(&mut canvas);
    }

    Ok(()) //Return OK / End of program
}