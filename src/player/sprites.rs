/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

use crate::player::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// True for rendering from low to high idx
/// False for high to low
#[derive(Resource)]
pub struct AnimationUpDown(pub bool);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &mut Player,
        &mut Transform,
    )>,
    mut animation_up: ResMut<AnimationUpDown>,
) {
    for (indices, mut timer, mut atlas, player, mut transform) in &mut query {
        timer.tick(time.delta());

        match player.state {
            PlayerState::Idle => {
                transform.rotation = Quat::from_rotation_x(0.0);
                if timer.just_finished() {
                    if animation_up.0 {
                        // animate from low to higher index
                        if atlas.index < indices.last {
                            atlas.index += 1
                        } else {
                            animation_up.0 = false;
                        }
                    } else {
                        // animate from higher to lower index
                        if atlas.index > indices.first {
                            atlas.index -= 1;
                        } else {
                            animation_up.0 = true;
                        }
                    }
                }
            }
            PlayerState::Running => {
                transform.rotation = Quat::from_rotation_x(0.0);
                if timer.just_finished() {
                    atlas.index = if atlas.index < SPRITE_RUN_IDX.0 {
                        SPRITE_RUN_IDX.0
                    } else if atlas.index >= SPRITE_RUN_IDX.1 {
                        SPRITE_IDLE_IDX.0
                    } else {
                        atlas.index + 1
                    }
                }
            }
            PlayerState::Jumping => {
                atlas.index = 9;
            }
            PlayerState::Dead => {
                atlas.index = 6;
                transform.rotation = Quat::from_rotation_x(std::f32::consts::PI);
            }
        }

        debug!("indice {}", atlas.index);
    }
}
