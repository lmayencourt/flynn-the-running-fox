/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use rand::prelude::*;

use crate::physics::{Collider, RigidBody};

/// World size definition
const WORLD_HEIGHT: f32 = 800.0;
const WORLD_TOP: f32 = WORLD_HEIGHT / 2.0;
const WORLD_BOTTOM: f32 = -WORLD_TOP;
// Use the golden ration here for the world size
const WORLD_WIDTH: f32 = WORLD_HEIGHT * 1.618;
const WORLD_RIGHT: f32 = WORLD_WIDTH / 2.0;
const WORLD_LEFT: f32 = -WORLD_RIGHT;

// Obstacle constant
const OBSTACLE_GAP_SIZE: f32 = 200.0;
const OBSTACLE_WIDTH: f32 = 20.0;
const OBSTACLE_SPEED: f32 = 120.0;

const OBSTACLE_SPAWN_SPEED: f32 = 2.0;

// const OBSTACLE_

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
struct ObstacleSpawnTimer {
    timer: Timer,
}
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.insert_resource(ObstacleSpawnTimer{timer: Timer::from_seconds(OBSTACLE_SPAWN_SPEED, TimerMode::Repeating)});
        app.add_systems(FixedUpdate, update_world);
    }
}

fn setup(mut commands: Commands) {
    // Create the first obstacles
    spawn_obstacle(&mut commands, 350.0);
}

fn update_world(
    mut commands: Commands,
    mut obstacles_query: Query<(&Transform, Entity), With<Obstacle>>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
    time: Res<Time>,
) {
    for (transform, entity) in obstacles_query.iter_mut() {
        if transform.translation.x < WORLD_LEFT {
            info!("Remove wall");
            commands.entity(entity).despawn();
        }
    }

    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.finished() {
        let mut gap_pos: f32 = rand::random::<f32>() * WORLD_HEIGHT;
        if gap_pos < OBSTACLE_GAP_SIZE {
            gap_pos = OBSTACLE_GAP_SIZE;
        } else if gap_pos > WORLD_HEIGHT - OBSTACLE_GAP_SIZE {
            gap_pos = WORLD_HEIGHT - OBSTACLE_GAP_SIZE;
        }
        spawn_obstacle(&mut commands, gap_pos);
    }
}

fn spawn_obstacle(commands: &mut Commands, gap_position: f32) {
    // Obstacles are composed of two walls, with a gap somewhere
    let top_wall_height: f32 = WORLD_HEIGHT - (gap_position + OBSTACLE_GAP_SIZE / 2.0);
    let top_wall_y_pos = WORLD_TOP - top_wall_height / 2.0;
    let bottom_wall_height: f32 = WORLD_HEIGHT - top_wall_height - OBSTACLE_GAP_SIZE;
    let bottom_wall_y_pos = WORLD_BOTTOM + bottom_wall_height / 2.0;

    info!("top height {} at {}", top_wall_height, top_wall_y_pos);
    info!("bottom {} at {}", bottom_wall_height, bottom_wall_y_pos);

    commands.spawn((
        SpriteBundle {
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
        RigidBody {
            // Constant speed, no gravity
            position: Vec2::new(WORLD_RIGHT, top_wall_y_pos),
            velocity: Vec2::new(-OBSTACLE_SPEED, 0.0),
            ..Default::default()
        },
    ));

    commands.spawn((
        SpriteBundle {
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
        RigidBody {
            // Constant speed, no gravity
            position: Vec2::new(WORLD_RIGHT, bottom_wall_y_pos),
            velocity: Vec2::new(-OBSTACLE_SPEED, 0.0),
            ..Default::default()
        },
    ));
}
