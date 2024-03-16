/* SPDX-License-Identifier: MIT
 *
 * This files uses concept from code:
 * https://github.com/mixandjam/Celeste-Movement/blob/master/Assets/Scripts/Movement.cs
 * Copyright (c) 2019 André Cardoso
 * 
 * Copyright (c) 2024 Louis Mayencourt
 */

/* Inspiration from Celeste moves set
 * - https://www.youtube.com/watch?v=yorTG9at90g
 * - https://www.youtube.com/watch?v=STyY26a_dPY
 */

use bevy::ecs::query;
use bevy::prelude::*;

use crate::player::*;
use crate::controller::*;

const RUNNING_SPEED: f32 = 200.0;
const JUMP_HEIGHT: f32 = 2.5 * SPRITE_SIZE;

const FALL_SPEED: f32 = 80.0;

pub fn player_movement (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Controller, &mut Player)>,
    time: Res<Time>
) {
    let (mut transform, controller, mut player) = query.single_mut();

    transform.translation.x += controller.direction.x * RUNNING_SPEED * time.delta_seconds();

    info!("Player state {:?}", player.state);
    info!("Player attitude {:?}", player.attitude);
    info!("Control state {:?}", controller.direction);
    match player.attitude {
        PlayerAttitude::Grounded => {
            player.state = PlayerState::Running;

            // Can only jump if on the ground
            if controller.action == Action::Jump {
                player.state = PlayerState::Jumping;
                transform.translation.y += JUMP_HEIGHT;
            }
        },
        PlayerAttitude::InAir => {
            // when in air, gravity applies
            transform.translation.y += -FALL_SPEED * time.delta_seconds();
        },
    }
}