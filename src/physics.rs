/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::math::bounding::{Aabb2d, RayCast2d};
use bevy::prelude::*;

use crate::player::{Player, PlayerAttitude, SPRITE_SIZE};
use crate::world::Ground;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Debug)]
pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Default for RigidBody {
    fn default() -> Self {
        RigidBody {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
        }
    }
}

pub fn BodiesMovement (
    mut query: Query<(&mut RigidBody, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut body, mut transform) in query.iter_mut() {
        let delta_t = time.delta_seconds();
        // Apply MRUA equation
        body.position.x += body.velocity.x * delta_t;
        body.position.y += body.velocity.y * delta_t;
        body.position.x += 0.5 * body.acceleration.x * delta_t * delta_t;
        body.position.y += 0.5 * body.acceleration.y * delta_t * delta_t;
        body.velocity.x += body.acceleration.x * delta_t;
        body.velocity.y += body.acceleration.y * delta_t;

        // Apply the new position to the sprite
        transform.translation.x = body.position.x;
        transform.translation.y = body.position.y;

        // info!("New RigidBody pos is {}, {}", transform.translation.x, transform.translation.y);
        info!("New RigidBody is {:?}", body);
    }
}

pub fn collision (
    mut gizmos: Gizmos,
    mut ground_query: Query<(&Transform), (With<Collider>, With<Ground>)>,
    mut player_query: Query<(&Transform, &mut Player)>,
) {
    let (player_transform, mut player) = player_query.single_mut();
    let ground = ground_query.single_mut();

    // let player_box = Aabb2d::new(player_transform.translation.truncate(), player_transform.scale.truncate()/2.0);
    let ground_box = Aabb2d::new(ground.translation.truncate(), ground.scale.truncate()/2.0);

    // A ray is an infinitely long line 
    let ground_ray = Ray2d::new(player_transform.translation.truncate(), Vec2::NEG_Y);
    // Ray cat have uses the origin and direction from ray, but have a finite length
    let ray_cast = RayCast2d::from_ray(ground_ray, SPRITE_SIZE * player_transform.scale.y/2.0);

    if let Some(point) = ray_cast.aabb_intersection_at(&ground_box) {
        info!("Collision on ray at {}", point);
        player.attitude = PlayerAttitude::Grounded;

        gizmos.circle_2d(ray_cast.ray.origin + *ray_cast.ray.direction * point, 10.0, Color::RED);
    } else {
        player.attitude = PlayerAttitude::InAir;
    }

    // Debug physics by displaying gizmos line
    // let player_pos = player.translation.truncate();
    // gizmos.ray_2d(player.translation.truncate(), Vec2::Y * SPRITE_SIZE * -player.scale.truncate()/2.0, Color::GREEN);
    // gizmos.ray_2d(ground_ray.origin, ground_ray.direction.to * SPRITE_SIZE * -player.scale.truncate()/2.0, Color::GRAY);

    gizmos.ray_2d(ground_ray.origin, *ground_ray.direction * ray_cast.max, Color::GRAY);
    // gizmos.line_2d(ray_cast.ray.origin, ray_cast.ray.origin + *ray_cast.ray.direction * ray_cast.max, Color::GRAY);
}