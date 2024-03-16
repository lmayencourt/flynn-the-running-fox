/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

use crate::physics::{Collider, RigidBody};

const WORLD_HEIGHT: f32 = 800.0;
const WORLD_TOP: f32 = WORLD_HEIGHT/2.0;
const WORLD_BOTTOM: f32 = -WORLD_TOP;
// Use the golden ration here for the world size
const WORLD_WIDTH: f32 = WORLD_HEIGHT * 1.618;
const WORLD_RIGHT: f32 = WORLD_WIDTH/2.0;
const WORLD_LEFT: f32 = -WORLD_RIGHT;

const OBSTACLE_GAP_SIZE: f32 = 200.0;
const OBSTACLE_WIDTH: f32 = 20.0;
const OBSTACLE_SPEED: f32 = 120.0;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Obstacle;

pub fn setup(
    mut commands: Commands,
) {
    // Create the first obstacles
    spawn_obstacle(&mut commands,  350.0);
}

fn spawn_obstacle(commands: &mut Commands, gap_position: f32) {
    // Obstacles are composed of two walls, with a gap somewhere
    let top_wall_height: f32 = WORLD_HEIGHT - (gap_position + OBSTACLE_GAP_SIZE/2.0);
    let top_wall_y_pos = WORLD_TOP-top_wall_height/2.0;
    let bottom_wall_height: f32 = WORLD_HEIGHT - top_wall_height - OBSTACLE_GAP_SIZE;
    let bottom_wall_y_pos = WORLD_BOTTOM + bottom_wall_height/2.0;

    info!("top height {} at {}", top_wall_height, top_wall_y_pos);
    info!("bottom {} at {}", bottom_wall_height, bottom_wall_y_pos);

    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(WORLD_RIGHT, top_wall_y_pos, 0.0),
            scale: Vec3::new(OBSTACLE_WIDTH, top_wall_height, 0.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
        },
        Obstacle,
        Collider,
        ShowAabbGizmo{color:None},
        RigidBody {
            // Constant speed, no gravity
            position: Vec2::new(WORLD_RIGHT, top_wall_y_pos),
            velocity: Vec2::new(-OBSTACLE_SPEED, 0.0),
            ..Default::default()
        }
    ));

    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(WORLD_RIGHT, bottom_wall_y_pos, 0.0),
            scale: Vec3::new(OBSTACLE_WIDTH, bottom_wall_height, 0.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    },
    Obstacle,
    Collider,
    ShowAabbGizmo{color:None},
    RigidBody {
        // Constant speed, no gravity
        position: Vec2::new(WORLD_RIGHT, bottom_wall_y_pos),
        velocity: Vec2::new(-OBSTACLE_SPEED, 0.0),
        ..Default::default()
    }
    ));
}