use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod mobs;

pub const PLAYER_MOVE_SPEED: f32 = 500.0;
pub const PLAYER_ROTATE_SPEED: f32 = 50.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Health {
    current_health: f32,
    health_max: HealthMax,
}

pub enum HealthMax {
    Player,
    Zombie,
}

pub struct Chunk {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();

    println!("End")
}

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let window = window_query.get_single().unwrap();

    //Player
    commands.spawn(
        (Camera3dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
         Player {},
         Health {
             current_health: from_health_max(HealthMax::Player),
             health_max: HealthMax::Player,
         },
        ),
    );
}

pub fn spawn_hostile_mob(mut commands: Commands) {
    commands.spawn((
        mobs::HostileMob {
            mob_type: mobs::HostileType::Zombie,
            state: mobs::HostileMobState::Idle,
        },
        Health {
            current_health: from_health_max(HealthMax::Zombie),
            health_max: HealthMax::Zombie,
        }
    ));
}

pub fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>, time: Res<Time>) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut around_y: f32 = 0.0;
        let mut around_x: f32 = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            around_y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            around_y += -1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            around_x += -1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            around_x += 1.0;
        }

        let t_up: Direction3d = transform.up();
        let t_right: Direction3d = transform.right();
        transform.rotate_local_axis(*t_up, around_y * PLAYER_ROTATE_SPEED * time.delta_seconds());
        transform.rotate_local_axis(*t_right, around_x * PLAYER_ROTATE_SPEED * time.delta_seconds());

        let mut f: f32 = 0.0;
        let mut r: f32 = 0.0;

        if keyboard_input.pressed(KeyCode::KeyD) {
            r += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            r += -1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            f += -1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            f += 1.0;
        }

        let direction: Vec3 = (transform.forward() * f + transform.right() * r).normalize();

        transform.translation += direction * PLAYER_MOVE_SPEED * time.delta_seconds();

        confine_player_movement(player_query, window_query)
    }
}

pub fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let min: f32 = half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        if translation.x < min {
            translation.x = min;
        }
        if translation.y < min {
            translation.y = min;
        }

        if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn check_health(mut health: Query<&mut Health>) {}

pub fn enemy_update(mut enemy_transform_query: Query<&mut Transform, With<mobs::HostileMob>>) {}

pub fn from_health_max(max: HealthMax) -> f32 {
    match max {
        HealthMax::Player => 10.0,
        HealthMax::Zombie => 10.0,
    }
}

pub fn size_from_hostile_type(hostile_mob: mobs::HostileType) -> (f32, f32, f32) {
    match hostile_mob {
        mobs::HostileType::Zombie => (17.5, 17.5, 17.5)
    }
}

pub fn size_from_friendly_type(friendly_mob: mobs::FriendlyType) -> (f32, f32, f32) {
    match friendly_mob {
        mobs::FriendlyType::Chicken => (7.5, 7.5, 7.5),
    }
}