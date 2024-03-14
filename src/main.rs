/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

mod player;

use controller::*;
use player::*;

const BIRD_SIZE: Vec3 = Vec3::new(50.0, 50.0, 0.0);

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        // .add_plugins(DefaultPlugins)
        // .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, player::setup)
        .add_systems(FixedUpdate, player::controller::keyboard_inputs)
        .add_systems(FixedUpdate, player::movement::player_movement)
        .add_systems(Update, player::sprites::animate_sprite)
        .run();
}

// fn setup(
//     mut commands: Commands,
// ) {
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn((SpriteBundle {
//         transform: Transform {
//             translation: Vec3::new(0.0, 0.0, 0.0),
//             scale: BIRD_SIZE,
//             ..default()
//         },
//         sprite: Sprite {
//             color: Color::rgb(0.0, 0.0, 1.0),
//             ..default()
//         },
//         ..default()
//     },
//     Player{state:PlayerState::Running}));
// }