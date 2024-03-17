/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

pub mod controller;
pub mod movement;
pub mod sprites;

use crate::{physics::{Collider, RigidBody}, ApplicationState};

use controller::*;
use sprites::*;

pub const SPRITE_HEIGHT: f32 = 15.0;
pub const SPRITE_WIDTH: f32 = 24.0;

#[derive(Component)]
pub struct Player {
    state: PlayerState,
    pub attitude: PlayerAttitude,
}

#[derive(Debug)]
enum PlayerState {
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
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, controller::keyboard_inputs
            .run_if(in_state(ApplicationState::InGame)));
        app.add_systems(FixedUpdate, movement::player_movement
            .run_if(in_state(ApplicationState::InGame)));
        app.add_systems(FixedUpdate, movement::collide_event_handler
            .run_if(in_state(ApplicationState::InGame)));
        app.add_systems(Update, sprites::animate_sprite);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("fox.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
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
            state: PlayerState::Running,
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
