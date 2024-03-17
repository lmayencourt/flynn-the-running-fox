/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::{gizmos, prelude::*};

use crate::player::Player;
use crate::world::Obstacle;

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

#[derive(Event, Default)]
pub struct CollideEvent;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollideEvent>();
        app.add_systems(FixedUpdate, bodies_movement);
        app.add_systems(FixedUpdate, collision);
    }
}

fn bodies_movement(mut query: Query<(&mut RigidBody, &mut Transform)>, time: Res<Time>) {
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
    }
}

fn collision(
    mut obstacles_query: Query<&Transform, With<Obstacle>>,
    mut player_query: Query<&Transform, With<Player>>,
    mut collision_events: EventWriter<CollideEvent>,
    mut gizmos: Gizmos,
) {
    let player_transform = player_query.single_mut();
    for obstacle in obstacles_query.iter_mut() {
        let player_box = Aabb2d::new(
            player_transform.translation.truncate(),
            player_transform.scale.truncate() * 6.0,
        );
        let obstacle_box = Aabb2d::new(
            obstacle.translation.truncate(),
            obstacle.scale.truncate() / 2.0,
        );

        if player_box.intersects(&obstacle_box) {
            collision_events.send_default();
        }

        // gizmos.rect_2d(player_box.center(), 0.0, player_box.half_size() *2.0, Color::GRAY);
    }
}
