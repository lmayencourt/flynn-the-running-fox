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

use bevy::prelude::*;

use crate::player::*;
use crate::controller::*;

const RUNNING_SPEED: f32 = 200.0;

pub fn player_movement (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Controller, &mut Player)>,
    time: Res<Time>
) {
    let (mut transform, controller, mut player) = query.single_mut();

    transform.translation.x += controller.direction.x * RUNNING_SPEED * time.delta_seconds();

    info!("Player state {:?}", player.state);
    info!("Control state {:?}", controller.direction);
    match player.state {
        PlayerState::Running => {
            if controller.action == Action::Jump {
                player.state = PlayerState::Jumping;
                // transform.translation.x += 10.0 * time.delta_seconds();
            }
        },
        PlayerState::Jumping => {
            player.jump_timer.tick(time.delta());
            if player.jump_timer.finished() {
                player.state = PlayerState::Running;
            } else {
                transform.translation.y += 20.0 * time.delta_seconds();
            }
        }
    }
}