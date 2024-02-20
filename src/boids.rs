use bevy::prelude::{Component, Plugin, Resource, Update, Vec2, Vec3};

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

#[derive(Component, Clone, Copy)]
pub struct BoidCollisionGroup(u32);

bitflags::bitflags! {
    impl BoidCollisionGroup: u32 {
        /// The group n°1.
        const GROUP_1 = 1 << 0;
        /// The group n°2.
        const GROUP_2 = 1 << 1;
        /// The group n°3.
        const GROUP_3 = 1 << 2;
        /// The group n°4.
        const GROUP_4 = 1 << 3;
        /// The group n°5.
        const GROUP_5 = 1 << 4;
        /// The group n°6.
        const GROUP_6 = 1 << 5;
        /// The group n°7.
        const GROUP_7 = 1 << 6;
        /// The group n°8.
        const GROUP_8 = 1 << 7;
        /// The group n°9.
        const GROUP_9 = 1 << 8;
        /// The group n°10.
        const GROUP_10 = 1 << 9;
        /// The group n°11.
        const GROUP_11 = 1 << 10;
        /// The group n°12.
        const GROUP_12 = 1 << 11;
        /// The group n°13.
        const GROUP_13 = 1 << 12;
        /// The group n°14.
        const GROUP_14 = 1 << 13;
        /// The group n°15.
        const GROUP_15 = 1 << 14;
        /// The group n°16.
        const GROUP_16 = 1 << 15;
        /// The group n°17.
        const GROUP_17 = 1 << 16;
        /// The group n°18.
        const GROUP_18 = 1 << 17;
        /// The group n°19.
        const GROUP_19 = 1 << 18;
        /// The group n°20.
        const GROUP_20 = 1 << 19;
        /// The group n°21.
        const GROUP_21 = 1 << 20;
        /// The group n°22.
        const GROUP_22 = 1 << 21;
        /// The group n°23.
        const GROUP_23 = 1 << 22;
        /// The group n°24.
        const GROUP_24 = 1 << 23;
        /// The group n°25.
        const GROUP_25 = 1 << 24;
        /// The group n°26.
        const GROUP_26 = 1 << 25;
        /// The group n°27.
        const GROUP_27 = 1 << 26;
        /// The group n°28.
        const GROUP_28 = 1 << 27;
        /// The group n°29.
        const GROUP_29 = 1 << 28;
        /// The group n°30.
        const GROUP_30 = 1 << 29;
        /// The group n°31.
        const GROUP_31 = 1 << 30;
        /// The group n°32.
        const GROUP_32 = 1 << 31;

        /// All of the groups.
        const ALL = u32::MAX;
        /// None of the groups.
        const NONE = 0;
    }
}

impl Default for BoidCollisionGroup {
    fn default() -> Self {
        Self::ALL
    }
}

#[derive(Component)]
pub enum BoidCollider {
    Box(Vec3),
    Sphere(f32),
    Rect(Vec2),
    Circle(f32),
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

impl BoidBorder {
    fn calc_avoidance(&self, position: Vec3, boid: &Boid) -> Vec3 {
        // returns `point` normalized to [0, 1] range, allowing overflow for increased strength.
        fn normalize(point: f32, start: f32, end: f32) -> f32 {
            (point - start) / (end - start).clamp(0.0, f32::MAX)
        }

        let mut movement_vector = Vec3::ZERO;

        if let Some((left, margin)) = self.left {
            if position.x < left + margin {
                let normalized_point = 1.0 - normalize(position.x, left, left + margin);

                movement_vector.x += boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((right, margin)) = self.right {
            if position.x > right - margin {
                let normalized_point = normalize(position.x, right - margin, right);

                movement_vector.x -= boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((top, margin)) = self.top {
            if position.y > top - margin {
                let normalized_point = normalize(position.y, top - margin, top);

                movement_vector.y -= boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((bottom, margin)) = self.bottom {
            if position.y < bottom + margin {
                let normalized_point = 1.0 - normalize(position.y, bottom, bottom + margin);

                movement_vector.y += boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((front, margin)) = self.front {
            if position.z < front + margin {
                let normalized_point = 1.0 - normalize(position.z, front, front + margin);

                movement_vector.z += boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((back, margin)) = self.back {
            if position.z > back - margin {
                let normalized_point = normalize(position.z, back - margin, back);

                movement_vector.z -= boid.turning_strength.border * normalized_point;
            }
        }

        movement_vector
    }
}

mod systems {
    use bevy::{
        prelude::{Color, Entity, Gizmos, Query, Res, Transform, Vec2},
        time::Time,
    };

    use super::*;

    pub(super) fn handle_boid_movement(
        time: Res<Time>,
        mut boid_query: Query<(
            &mut Transform,
            &mut Boid,
            Option<&BoidBorder>,
            Option<&BoidCollisionGroup>,
            Entity,
        )>,
    ) {
        let boids = boid_query
            .iter()
            .map(|(transform, boid, _, collision_group, entity)| {
                (
                    *transform,
                    *boid,
                    collision_group.and_then(|cg| Some(*cg)),
                    entity,
                )
            })
            .collect::<Vec<_>>();

        for (mut transform, mut boid, border, collision_group, entity) in boid_query.iter_mut() {
            let mut movement_vector = Vec3::ZERO;

            let mut separation_vector = Vec3::ZERO;
            let mut average_velocity = Vec3::ZERO;
            let mut average_position = Vec3::ZERO;
            let mut neighbouring_boids = 0;

            for (
                neighbour_transform,
                neighbour_boid,
                neighbour_collision_group,
                neighbour_entity,
            ) in &boids
            {
                if *neighbour_entity == entity {
                    continue;
                }

                if let Some(collision_group) = collision_group {
                    if let Some(neighbour_collision_group) = neighbour_collision_group {
                        if !collision_group.intersects(*neighbour_collision_group) {
                            continue;
                        }
                    }
                }

                let distance = transform
                    .translation
                    .distance(neighbour_transform.translation);

                if distance <= boid.view_config.view_range {
                    if distance < boid.view_config.protected_range {
                        let normalized_diff =
                            (neighbour_transform.translation - transform.translation).normalize();
                        let strength = boid.view_config.protected_range - distance;

                        separation_vector -= normalized_diff * strength;
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

            if let Some(border) = border {
                movement_vector += border.calc_avoidance(transform.translation, &boid);
            }

            let new_velocity = boid.velocity + movement_vector;

            boid.velocity = if new_velocity.length_squared() > 0.0 {
                new_velocity.clamp_length(boid.speed.min, boid.speed.max)
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
            let forward = transform.forward();

            match config.space {
                BoidSpace::TwoDimensional => transform.look_to(*forward, boid.velocity.normalize()),
                BoidSpace::ThreeDimensional => {
                    transform.look_to(boid.velocity.normalize(), *forward)
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
