/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

pub mod controller;
pub mod movement;
pub mod sprites;

use crate::{physics::Collider, RigidBody};

use sprites::*;
use controller::*;

pub const SPRITE_SIZE: f32 = 24.0;

#[derive(Component)]
pub struct Player {
    state: PlayerState,
    pub attitude: PlayerAttitude,
    jump_timer: Timer,
}

#[derive(Debug)]
enum PlayerState {
    Running,
    Jumping,
}

#[derive(Debug)]
pub enum PlayerAttitude {
    Grounded,
    InAir,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("fox.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 6, 1, None, None);
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
            transform: Transform::from_xyz(0.0, 40.0, 0.0)
                .with_scale(Vec3::splat(4.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player{
            state:PlayerState::Running,
            attitude:PlayerAttitude::Grounded,
            jump_timer:Timer::from_seconds(0.4, TimerMode::Repeating)
        },
        Controller{direction:Vec2::ZERO, action:Action::None},
        Collider,
        RigidBody {
            position: Vec2::new(0.0, 40.0),
            ..default()
        },
        // ShowAabbGizmo{color:None},
    ));
}