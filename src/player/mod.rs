/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

pub mod controller;
pub mod movement;
pub mod sprites;

use crate::{
    physics::{Collider, RigidBody},
    ApplicationState, RestartEvent,
};

use controller::*;
use sprites::*;

pub const SPRITE_HEIGHT: f32 = 15.0;
pub const SPRITE_WIDTH: f32 = 24.0;
pub const SPRITE_IDLE_IDX: (usize, usize) = (0, 5);
pub const SPRITE_RUN_IDX: (usize, usize) = (6, 11);

#[derive(Component)]
pub struct Player {
    state: PlayerState,
    pub attitude: PlayerAttitude,
}

#[derive(Debug)]
enum PlayerState {
    Idle,
    Running,
    Jumping,
    Dead,
}

#[derive(Debug)]
pub enum PlayerAttitude {
    InAir,
    InWall,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(sprites::AnimationUpDown(true));
        app.add_systems(Startup, setup);
        app.add_systems(Update, restart_event_handler);
        app.add_systems(
            FixedUpdate,
            controller::keyboard_inputs.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            movement::player_movement
                .after(controller::keyboard_inputs)
                .run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            movement::collide_event_handler.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(Update, sprites::animate_sprite);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("snow-fox-sprites.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT), 6, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 5 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_xyz(0.0, 40.0, 0.0).with_scale(Vec3::splat(4.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            state: PlayerState::Idle,
            attitude: PlayerAttitude::InAir,
            // jump_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
        },
        Controller {
            direction: Vec2::ZERO,
            action: Action::None,
        },
        Collider,
        RigidBody {
            position: Vec2::new(0.0, 40.0),
            ..default()
        },
        // ShowAabbGizmo { color: None },
    ));
}

pub fn restart_event_handler(events: EventReader<RestartEvent>, mut query: Query<&mut Player>) {
    if !events.is_empty() {
        info!("Restart Game!");
        let mut player = query.single_mut();
        player.attitude = PlayerAttitude::InAir;
    }
}
