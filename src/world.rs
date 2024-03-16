/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

use crate::physics::{Collider, RigidBody};

const GROUND_HEIGHT: f32 = 20.0;
const GROUND_SIZE: Vec3 = Vec3::new(250.0, GROUND_HEIGHT, 0.0);

const OBSTACLE_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Obstacle;

pub fn setup(
    mut commands: Commands,
) {
    // Ground
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -GROUND_HEIGHT, 0.0),
            scale: GROUND_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    },
    Ground,
    Collider));

    // Ground
    spawn_platform(&mut commands, -350.0, GROUND_HEIGHT, GROUND_SIZE.x, GROUND_SIZE.y);
    spawn_platform(&mut commands, 0.0, -GROUND_HEIGHT, GROUND_SIZE.x, GROUND_SIZE.y);
    spawn_platform(&mut commands, 350.0, -GROUND_HEIGHT, GROUND_SIZE.x, GROUND_SIZE.y);

    // Obstacle
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(200.0, 0.0, 0.0),
            scale: OBSTACLE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    },
    Obstacle,
    ));
}

fn spawn_platform(commands: &mut Commands, x: f32, y: f32, h: f32, l: f32) {
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::new(h, l, 0.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    },
    Ground,
    Collider));
}