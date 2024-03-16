/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::{
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod physics;
mod player;
mod world;

use player::PlayerPlugin;
use physics::PhysicsPlugin;
use world::WorldPlugin;

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        // Custom plugin and systems
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}
