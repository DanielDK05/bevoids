use bevy::prelude::{Component, Plugin, Resource, Update, Vec3};

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                systems::handle_boid_gizmos,
                systems::handle_boid_movement,
                systems::handle_boid_orientation,
            ),
        );
    }
}

#[derive(Resource)]
pub struct BoidsConfig {
    pub space: BoidSpace,
    pub debug: bool,
}

pub enum BoidSpace {
    TwoDimensional,
    ThreeDimensional,
}

#[derive(Clone, Copy)]
pub struct BoidTurningStrength {
    pub coherence: f32,
    pub separation: f32,
    pub alignment: f32,
    pub border: f32,
}

impl BoidTurningStrength {
    pub fn new(coherence: f32, separation: f32, alignment: f32, border: f32) -> Self {
        Self {
            coherence,
            separation,
            alignment,
            border,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoidViewConfig {
    pub fov: u32,
    pub protected_range: f32,
    pub view_range: f32,
}

impl BoidViewConfig {
    pub fn new(fov: u32, protected_range: f32, view_range: f32) -> Self {
        Self {
            fov,
            protected_range,
            view_range,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoidSpeed {
    min: f32,
    max: f32,
}

impl BoidSpeed {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Boid {
    pub speed: BoidSpeed,
    pub turning_strength: BoidTurningStrength,
    pub view_config: BoidViewConfig,
    velocity: Vec3,
}

impl Boid {
    pub fn new(
        speed: BoidSpeed,
        turning_strength: BoidTurningStrength,
        view_config: BoidViewConfig,
    ) -> Self {
        Self {
            speed,
            turning_strength,
            view_config,
            velocity: Vec3::ZERO,
        }
    }
}

#[derive(Component, Default)]
pub struct BoidBorder {
    pub top: Option<(f32, f32)>,
    pub bottom: Option<(f32, f32)>,
    pub left: Option<(f32, f32)>,
    pub right: Option<(f32, f32)>,
    pub front: Option<(f32, f32)>,
    pub back: Option<(f32, f32)>,
}

mod systems {
    use bevy::{
        prelude::{Color, Entity, Gizmos, Query, Res, Transform, Vec2},
        time::Time,
    };

    use super::*;

    pub(super) fn handle_boid_movement(
        time: Res<Time>,
        mut boid_query: Query<(&mut Transform, &mut Boid, Option<&BoidBorder>, Entity)>,
    ) {
        let boids = boid_query
            .iter()
            .map(|(transform, boid, _, entity)| (*transform, *boid, entity))
            .collect::<Vec<_>>();

        for (mut transform, mut boid, border, entity) in boid_query.iter_mut() {
            let mut movement_vector = Vec3::ZERO;

            let mut separation_vector = Vec3::ZERO;
            let mut average_velocity = Vec3::ZERO;
            let mut average_position = Vec3::ZERO;
            let mut neighbouring_boids = 0;

            for (neighbour_transform, neighbour_boid, neighbour_entity) in &boids {
                if *neighbour_entity == entity {
                    continue;
                }

                let distance = transform
                    .translation
                    .distance(neighbour_transform.translation);

                if distance <= boid.view_config.view_range {
                    if distance < boid.view_config.protected_range {
                        separation_vector -=
                            (neighbour_transform.translation - transform.translation).normalize()
                                * (boid.view_config.protected_range - distance);
                    }

                    average_velocity += neighbour_boid.velocity;
                    average_position += neighbour_transform.translation;

                    neighbouring_boids += 1;
                }
            }

            if neighbouring_boids > 0 {
                average_velocity /= neighbouring_boids as f32;
                average_position /= neighbouring_boids as f32;

                movement_vector +=
                    (average_velocity - boid.velocity) * boid.turning_strength.alignment;
                movement_vector +=
                    (average_position - transform.translation) * boid.turning_strength.coherence;
            }

            movement_vector += separation_vector * boid.turning_strength.separation;

            let mut border_turn_vector = Vec3::ZERO;
            if let Some(border) = border {
                fn normalize(point: f32, start: f32, end: f32) -> f32 {
                    (point - start) / (end - start).clamp(0.0, f32::MAX)
                }

                if let Some((left, margin)) = border.left {
                    if transform.translation.x < left + margin {
                        let normalized_point =
                            1.0 - normalize(transform.translation.x, left, left + margin);

                        border_turn_vector.x += boid.turning_strength.border * normalized_point;
                    }
                }
                if let Some((right, margin)) = border.right {
                    if transform.translation.x > right - margin {
                        let normalized_point =
                            normalize(transform.translation.x, right - margin, right);

                        border_turn_vector.x -= boid.turning_strength.border * normalized_point;
                    }
                }
                if let Some((top, margin)) = border.top {
                    if transform.translation.y > top - margin {
                        let normalized_point =
                            normalize(transform.translation.y, top - margin, top);

                        border_turn_vector.y -= boid.turning_strength.border * normalized_point;
                    }
                }
                if let Some((bottom, margin)) = border.bottom {
                    if transform.translation.y < bottom + margin {
                        let normalized_point =
                            1.0 - normalize(transform.translation.y, bottom, bottom + margin);

                        border_turn_vector.y += boid.turning_strength.border * normalized_point;
                    }
                }
                if let Some((front, margin)) = border.front {
                    if transform.translation.z < front + margin {
                        let normalized_point =
                            1.0 - normalize(transform.translation.z, front, front + margin);

                        border_turn_vector.z += boid.turning_strength.border * normalized_point;
                    }
                }
                if let Some((back, margin)) = border.back {
                    if transform.translation.z > back - margin {
                        let normalized_point =
                            normalize(transform.translation.z, back - margin, back);

                        border_turn_vector.z -= boid.turning_strength.border * normalized_point;
                    }
                }
                movement_vector += border_turn_vector;
            }

            boid.velocity = if (boid.velocity + movement_vector).length_squared() > 0.0 {
                (boid.velocity + movement_vector).clamp_length(boid.speed.min, boid.speed.max)
            } else {
                Vec3::X * boid.speed.min
            };

            transform.translation += boid.velocity * time.delta_seconds();
        }
    }

    pub(super) fn handle_boid_orientation(
        mut boid_query: Query<(&mut Transform, &Boid)>,
        config: Res<BoidsConfig>,
    ) {
        for (mut transform, boid) in boid_query.iter_mut() {
            match config.space {
                BoidSpace::TwoDimensional => {
                    let forward = transform.forward();
                    transform.look_to(forward, boid.velocity.normalize())
                }
                BoidSpace::ThreeDimensional => {
                    let forward = transform.forward();
                    transform.look_to(boid.velocity.normalize(), forward)
                }
            }
        }
    }

    pub(super) fn handle_boid_gizmos(
        config: Res<BoidsConfig>,
        boid_query: Query<(&Transform, &Boid)>,
        mut gizmos: Gizmos,
    ) {
        if config.debug {
            match config.space {
                BoidSpace::TwoDimensional => {
                    for (transform, boid) in boid_query.iter() {
                        for angle in
                            -(boid.view_config.fov as i32 / 2)..boid.view_config.fov as i32 / 2
                        {
                            let angle_radians = (angle as f32).to_radians();

                            gizmos.ray_2d(
                                transform.translation.truncate(),
                                transform
                                    .up()
                                    .truncate()
                                    .rotate(Vec2::from_angle(angle_radians))
                                    * boid.view_config.view_range,
                                Color::Rgba {
                                    red: 1.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 0.01,
                                },
                            );
                        }
                    }
                }
                BoidSpace::ThreeDimensional => {
                    for (transform, boid) in boid_query.iter() {
                        for angle in
                            -(boid.view_config.fov as i32) / 2..boid.view_config.fov as i32 / 2
                        {
                            let angle_radians = (angle as f32).to_radians();

                            gizmos.ray(
                                transform.translation,
                                transform
                                    .up()
                                    .truncate()
                                    .rotate(Vec2::from_angle(angle_radians))
                                    .extend(0.0)
                                    * boid.view_config.view_range,
                                Color::Rgba {
                                    red: 1.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 0.1,
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}
