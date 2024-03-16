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

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas, &mut Player)>,
) {
    for (indices, mut timer, mut atlas, mut player) in &mut query {
        timer.tick(time.delta());

        match player.state {
            PlayerState::Running => {
                if timer.just_finished() {
                    atlas.index = if atlas.index == indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            },
            PlayerState::Jumping => {
                atlas.index = 3;
            },
        }
    }
}