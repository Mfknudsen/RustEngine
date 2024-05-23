use std::sync::{mpsc, MutexGuard};

use sdl2::{event::Event, keyboard::Keycode};

use crate::{characters::player::Player, ControlMessage};

pub fn handle_event(
    tx: &mpsc::Sender<ControlMessage>,
    event: &Event,
    mut player_lock: MutexGuard<Player>,
) -> Result<(), String> {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            tx.send(ControlMessage::Break)
                .map_err(|e| format!("Failed to send ControlMessage::Break: {}", e))?;
        }

        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => {
            player_lock.update_input(Keycode::A, true);
        }
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => {
            player_lock.update_input(Keycode::D, true);
        }
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } => {
            player_lock.update_input(Keycode::Space, true);
        }

        Event::KeyUp {
            keycode: Some(Keycode::A),
            ..
        } => {
            player_lock.update_input(Keycode::A, false);
        }
        Event::KeyUp {
            keycode: Some(Keycode::D),
            ..
        } => {
            player_lock.update_input(Keycode::D, false);
        }

        _ => {}
    }
    Ok(())
}