/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::{
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod physics;
mod player;
mod world;

use player::PlayerPlugin;
use physics::PhysicsPlugin;
use world::WorldPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ApplicationState {
    LandingScreen,
    InGame,
    GameEnd,
}

#[derive(Component)]
struct MenuText;

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        // Custom plugin and systems
        .insert_state(ApplicationState::LandingScreen)
        .add_systems(Startup, menu_setup)
        .add_systems(Update, menu_control)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}

fn menu_setup (
    mut commands: Commands,
) {
    let text_style = TextStyle {
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Press \"Space\" to start", text_style),
            ..default()
        },
        MenuText,
    ));
}

fn menu_control(
    state: Res<State<ApplicationState>>,
    mut next_state: ResMut<NextState<ApplicationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Text, With<MenuText>>,
) {
    match state.get() {
        ApplicationState::LandingScreen => {
            if keyboard_input.pressed(KeyCode::Space) {
                next_state.set(ApplicationState::InGame);
            }
        },
        ApplicationState::InGame => {
            let mut text = query.single_mut();
            text.sections[0].value = "".to_string();
        },
        ApplicationState::GameEnd => {
            let mut text = query.single_mut();
            text.sections[0].value = "You died...".to_string();

            if keyboard_input.pressed(KeyCode::Space) {
                next_state.set(ApplicationState::InGame);
            }
        },
    }
}