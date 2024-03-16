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

use crate::physics::{CollideEvent};
use crate::player::*;

pub fn player_movement(
    mut query: Query<(&mut RigidBody, &Controller, &mut Player)>,
) {
    let (mut body, controller, mut player) = query.single_mut();

    info!("Player state {:?}", player.state);
    info!("Player attitude {:?}", player.attitude);
    info!("Control state {:?}", controller.direction);
    match player.attitude {
        PlayerAttitude::InAir => {
            player.state = PlayerState::Running;

            // Can only jump if on the ground
            if controller.action == Action::Jump {
                player.state = PlayerState::Jumping;
                body.velocity.y = 450.0;
            }
            // when in air, gravity applies
            body.acceleration.y = -1500.0;
        }
        PlayerAttitude::InWall => {
            player.state = PlayerState::Dead;
            body.velocity = Vec2::ZERO;
            body.acceleration = Vec2::ZERO;
        }
    }
}

pub fn collide_event_handler(events: EventReader<CollideEvent>, mut query: Query<&mut Player>) {
    if !events.is_empty() {
        info!("End of Game !");
        let mut player = query.single_mut();
        player.attitude = PlayerAttitude::InWall;
    }
}
