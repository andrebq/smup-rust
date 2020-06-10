extern crate specs;

use crate::types::{GameState};

pub use nalgebra::Vector2;

use nphysics2d::math::Velocity;
use nphysics2d::object::{
    BodyPartHandle, BodyStatus, ColliderDesc, DefaultBodyHandle, DefaultBodySet,
    DefaultColliderSet, RigidBodyDesc
};
use nphysics2d::joint::{DefaultJointConstraintSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use nphysics2d::force_generator::{DefaultForceGeneratorSet};
use specs::{System, Read};

pub type Vec2 = Vector2<f32>;
pub type Handle = DefaultBodyHandle;

pub struct PhysicsSystem {
    gworld: DefaultGeometricalWorld<f32>,
    mworld: DefaultMechanicalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    constraints: DefaultJointConstraintSet<f32>,
    forces: DefaultForceGeneratorSet<f32>,
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = Read<'a, GameState>;
    fn run(&mut self, gs: Self::SystemData) {
        self.step_for(gs.delta);
        println!("Last iteration: {}", self.mworld.counters.step_time)
    }
}

impl PhysicsSystem {

    pub fn new() -> PhysicsSystem {
        let mut sys = PhysicsSystem {
            gworld: DefaultGeometricalWorld::new(),
            mworld: DefaultMechanicalWorld::new(Vec2::new(0., 9.8)),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            constraints: DefaultJointConstraintSet::new(),
            forces: DefaultForceGeneratorSet::new(),
        };
        sys.mworld.counters.enable();
        sys.gworld.maintain(&mut sys.bodies, &mut sys.colliders);
        sys.mworld.maintain(&mut sys.gworld, &mut sys.bodies, &mut sys.colliders, &mut sys.constraints);
        sys
    }

    fn step_for(&mut self, _update_dt: f64) {
        self.mworld.step(
            &mut self.gworld,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.constraints,
            &mut self.forces);
    }

    pub fn add_box(&mut self, pos: Vec2) -> Handle {
        use ncollide2d::shape::{Cuboid, ShapeHandle};
        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(25., 25.)));
        let body = RigidBodyDesc::new()
            .translation(pos)
            .velocity(Velocity::linear(1., 0.))
            .status(BodyStatus::Kinematic)
            .build();
        let handle = self.bodies.insert(body);
        self.colliders.insert(
            ColliderDesc::new(shape)
                .density(1.)
                .build(BodyPartHandle(handle, 0)),
        );
        handle
    }
}
