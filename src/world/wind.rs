/* SPDX-License-Identifier: MIT
 *
 * This files uses concept from code:
 * https://github.com/abnormalbrain/bevy_particle_systems/tree/main
 *
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_particle_systems::*;

use super::{WORLD_BOTTOM, WORLD_HEIGHT, WORLD_RIGHT, WORLD_TOP, WORLD_WIDTH};

pub fn spawn_particle_system(mut commands: Commands) {
    commands
        // Add the bundle specifying the particle system itself.
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 1024,
                emitter_shape: EmitterShape::Line(Line {
                    length: WORLD_WIDTH,
                    angle: JitteredValue::jittered(std::f32::consts::PI, -0.1..0.1),
                }),
                spawn_rate_per_second: 50.0.into(),
                initial_speed: JitteredValue::jittered(150.0, -50.0..50.0),
                lifetime: JitteredValue::jittered(18.0, -2.0..2.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::WHITE, 0.0),
                    CurvePoint::new(Color::rgba(0.5, 0.5, 1.0, 0.0), 1.0),
                ])),
                initial_scale: JitteredValue::jittered(2.5, -1.0..1.0),
                looping: true,
                system_duration_seconds: 10.0,
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(WORLD_RIGHT, WORLD_BOTTOM + WORLD_HEIGHT / 2.0, 0.0),
            ..ParticleSystemBundle::default()
        })
        // Add the playing component so it starts playing. This can be added later as well.
        .insert(Playing);
}
