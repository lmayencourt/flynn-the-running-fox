/* SPDX-License-Identifier: MIT
 *
 * This files uses concept from code:
 * https://github.com/djeedai/bevy_hanabi
 *
 * Copyright (c) 2024 Louis Mayencourt
 */

// use bevy::{
//     log::LogPlugin,
//     prelude::*,
//     render::{
//         camera::ScalingMode, render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin,
//     },
//     sprite::{MaterialMesh2dBundle, Mesh2dHandle},
// };
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use super::{WORLD_HEIGHT, WORLD_RIGHT, WORLD_TOP, WORLD_WIDTH};

pub fn setup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a color gradient for the particles
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.5, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.5, 0.5, 1.0, 0.0));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere
    let init_pos = SetPositionSphereModifier {
          center: module.lit(Vec3::new(WORLD_RIGHT + WORLD_WIDTH/8.0, WORLD_TOP - WORLD_HEIGHT/4.0, 0.0)),
          radius: module.lit(WORLD_HEIGHT/2.0),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
    //   center: module.lit(Vec3::new(WORLD_RIGHT, WORLD_BOTTOM + WORLD_HEIGHT/2.0, 0.0)),
        center: module.lit(Vec3::ZERO),
        speed: module.lit(30.),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(30.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(-20., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        256,
        // Spawn at a rate of 10 particles per second
        Spawner::rate(10.0.into()),
        // Move the expression module into the asset
        module
    )
    .with_name("TheWind")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(Vec3::Y),
            ..Default::default()
        });
}
