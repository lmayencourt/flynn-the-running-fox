/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_particle_systems::ParticleSystemPlugin;

mod audio;
mod physics;
mod player;
mod world;

use audio::AudioPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ApplicationState {
    LandingScreen,
    InGame,
    GameEnding,
    GameEnd,
}

#[derive(Component)]
struct MenuText;

#[derive(Event, Default)]
pub struct RestartEvent;

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest()) // prevents blurry sprites
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy game".to_string(), // ToDo
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
            )
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_systems(Update, bevy::window::clo)
        // .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(ParticleSystemPlugin)
        // Custom plugin and systems
        .insert_state(ApplicationState::LandingScreen)
        .add_event::<RestartEvent>()
        .add_systems(Startup, menu_setup)
        .add_systems(Update, menu_control)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(AudioPlugin)
        .run();
}

fn menu_setup(mut commands: Commands) {
    let text_style = TextStyle { ..default() };

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
    mut event: EventWriter<RestartEvent>,
) {
    match state.get() {
        ApplicationState::LandingScreen => {
            if keyboard_input.pressed(KeyCode::Space) {
                next_state.set(ApplicationState::InGame);
            }
        }
        ApplicationState::InGame => {
            let mut text = query.single_mut();
            text.sections[0].value = "".to_string();
        }
        ApplicationState::GameEnding => {
            let mut text = query.single_mut();
            text.sections[0].value = "You died...".to_string();
        }
        ApplicationState::GameEnd => {
            let mut text = query.single_mut();
            text.sections[0].value = "Press \"Space\" to restart!".to_string();

            if keyboard_input.pressed(KeyCode::Space) {
                next_state.set(ApplicationState::InGame);
                event.send_default();
            }
        }
    }
}
