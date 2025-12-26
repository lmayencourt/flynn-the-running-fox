/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Define the controls required to play the game
use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Action {
    None,
    Jump,
}

#[derive(Component)]
pub struct Controller {
    pub direction: Vec2,
    pub action: Action,
}

/// Controller implementation for keyboard
pub fn keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut query: Query<&mut Controller>,
) {
    let mut controller = query.single_mut();

    let touche = touches.first_pressed_position();
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) || touche.is_some() {
        // controller.direction.y = 1.0;
        controller.action = Action::Jump;
    } else {
        controller.action = Action::None;
    }
}
