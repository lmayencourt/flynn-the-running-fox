/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::{prelude::*, text};

use crate::{
    physics::{Collider, RigidBody, CollideEvent, CollideWith},
    ApplicationState, RestartEvent
};

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
const OBSTACLE_DESPAWN_SPEED: f32 = 0.20;

// const OBSTACLE_

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
struct ObstacleSpawnTimer {
    timer: Timer,
}

#[derive(Resource)]
struct ObstacleDespawnTimer {
    timer: Timer,
}

#[derive(Resource)]
struct ScoreBoard {
    score: u32,
}

#[derive(Component)]
struct ScoreBoardUi;

#[derive(Component)]
pub struct Waypoint;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObstacleSpawnTimer {
            timer: Timer::from_seconds(OBSTACLE_SPAWN_SPEED, TimerMode::Repeating),
        });
        app.insert_resource(ObstacleDespawnTimer {
            timer: Timer::from_seconds(OBSTACLE_DESPAWN_SPEED, TimerMode::Repeating),
        });
        app.insert_resource(ScoreBoard{score:0});
        app.add_systems(Startup, setup_world);
        app.add_systems(
            Update,
            update_world.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update,
            clear_world.run_if(in_state(ApplicationState::GameEnding))
        );
        app.add_systems(
            Update,
            collide_event_handler.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update,
            restart_event_handler.run_if(in_state(ApplicationState::GameEnd))
        );
    }
}

fn setup_world(mut commands: Commands) {
    // Top world border
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(WORLD_LEFT + WORLD_WIDTH/2.0, WORLD_TOP, 0.0),
                scale: Vec3::new(WORLD_WIDTH, OBSTACLE_WIDTH, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Collider,
    ));

    // Bottom world border
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(WORLD_LEFT + WORLD_WIDTH/2.0, WORLD_BOTTOM, 0.0),
                scale: Vec3::new(WORLD_WIDTH, OBSTACLE_WIDTH, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Collider,
    ));

    // Spawn Scoreboard
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Score: ", TextStyle { ..default() }),
                TextSection::from_style(TextStyle { ..default() }),
            ]),
            transform: Transform {
                translation: Vec3::new(WORLD_LEFT +  80.0, WORLD_TOP - 80.0, 0.0),
                ..default()
            },
            ..default()
        },
        ScoreBoardUi,
    ));
}

fn update_world(
    mut commands: Commands,
    mut obstacles_query: Query<(&Transform, Entity), With<Obstacle>>,
    mut scorebard: ResMut<ScoreBoard>,
    mut query: Query<&mut Text, With<ScoreBoardUi>>,
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

    let mut text = query.single_mut();
    text.sections[1].value = scorebard.score.to_string();
}

fn clear_world(
    mut commands: Commands,
    mut query: Query<Entity, With<Obstacle>>,
    mut despawn_timer: ResMut<ObstacleDespawnTimer>,
    mut next_state: ResMut<NextState<ApplicationState>>,
    time: Res<Time>,
) {
    despawn_timer.timer.tick(time.delta());
    if despawn_timer.timer.finished() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
            break;
        }
    }

    if query.is_empty() {
        next_state.set(ApplicationState::GameEnd);
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

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(WORLD_RIGHT, WORLD_BOTTOM + gap_position, 0.0),
                scale: Vec3::new(OBSTACLE_WIDTH, OBSTACLE_GAP_SIZE, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgba(0.75, 0.75, 0.75, 0.5),
                ..default()
            },
            ..default()
        },
        RigidBody {
            // Constant speed, no gravity
            position: Vec2::new(WORLD_RIGHT, WORLD_BOTTOM + gap_position),
            velocity: Vec2::new(-OBSTACLE_SPEED, 0.0),
            ..Default::default()
        },
        Waypoint,
    ));
}

fn collide_event_handler(
    mut events: EventReader<CollideEvent>,
    mut scorebard: ResMut<ScoreBoard>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let CollideWith::Waypoint(entity) = event.other {
            scorebard.score += 1;
            commands.entity(entity).despawn();
        }
    }
}

pub fn restart_event_handler(
    events: EventReader<RestartEvent>,
    mut scorebard: ResMut<ScoreBoard>,
) {
    scorebard.score = 0;
}