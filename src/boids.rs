use bevy::prelude::{Component, Plugin, Resource};

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
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

pub struct BoidTurningConfig {
    pub coherence: f32,
    pub separation: f32,
    pub alignment: f32,
}

impl BoidTurningConfig {
    pub fn new(coherence: f32, separation: f32, alignment: f32) -> Self {
        Self {
            coherence,
            separation,
            alignment,
        }
    }
}

pub struct BoidViewConfig {
    pub range: f32,
    pub distance: f32,
}

impl BoidViewConfig {
    pub fn new(range: f32, distance: f32) -> Self {
        Self { range, distance }
    }
}

#[derive(Component)]
pub struct Boid {
    speed: f32,
    turning_config: BoidTurningConfig,
    view_config: BoidViewConfig,
}

impl Boid {
    pub fn new(speed: f32, turning_config: BoidTurningConfig, view_config: BoidViewConfig) -> Self {
        Self {
            speed,
            turning_config,
            view_config,
        }
    }
}

mod systems {}
