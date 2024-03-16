/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::{gizmos::{self, aabb::AabbGizmoPlugin}, math::vec2, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod physics;
mod world;

use player::*;
use physics::*;
use world::*;

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        // .add_plugins(DefaultPlugins)
        // .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, world::setup)
        .add_systems(Startup, player::setup)
        .add_systems(FixedUpdate, player::controller::keyboard_inputs)
        .add_systems(FixedUpdate, player::movement::player_movement)
        .add_systems(FixedUpdate, physics::BodiesMovement)
        .add_systems(FixedUpdate, physics::collision)
        .add_systems(Update, player::sprites::animate_sprite)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}