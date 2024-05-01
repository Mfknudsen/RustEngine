use std::f32::consts::PI;

use bevy::{core_pipeline::fxaa::Fxaa, prelude::*};

mod debug;
mod voxel;

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Engine".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(voxel::VoxelWorldPlugin)
        .add_plugins(debug::DebugUIPlugins)
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((Player, Camera3dBundle{
        projection: Projection::Perspective(PerspectiveProjection{
            fov: PI / 2.0,
            far: 2048.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(2.0, 160.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }))
        .insert(voxel::player::PlayerController::default())
        .insert(Fxaa::default())
        .insert(bevy_atmosphere::plugin::AtmosphereCamera::default());
}