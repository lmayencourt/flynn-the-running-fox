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
use crate::RigidBody;

const RUNNING_SPEED: f32 = 200.0;
const JUMP_HEIGHT: f32 = 2.5 * SPRITE_SIZE;

const FALL_SPEED: f32 = 80.0;

pub fn player_movement (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut RigidBody, &Controller, &mut Player)>,
    time: Res<Time>
) {
    let (mut body, controller, mut player) = query.single_mut();

    body.velocity.x = controller.direction.x * RUNNING_SPEED;

    info!("Player state {:?}", player.state);
    info!("Player attitude {:?}", player.attitude);
    info!("Control state {:?}", controller.direction);
    match player.attitude {
        PlayerAttitude::Grounded => {
            player.state = PlayerState::Running;
            body.acceleration.y = 0.0;
            body.velocity.y = 0.0;

            // Can only jump if on the ground
            if controller.action == Action::Jump {
                player.state = PlayerState::Jumping;
                body.velocity.y = 100.0;
            }
        },
        PlayerAttitude::InAir => {
            // when in air, gravity applies
            body.acceleration.y = -200.0;
        },
    }
}