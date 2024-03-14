/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

mod player;
mod world;

use player::*;
use world::*;

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        // .add_plugins(DefaultPlugins)
        // .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, world::setup)
        .add_systems(Startup, player::setup)
        .add_systems(FixedUpdate, player::controller::keyboard_inputs)
        .add_systems(FixedUpdate, player::movement::player_movement)
        .add_systems(Update, player::sprites::animate_sprite)
        .run();
}