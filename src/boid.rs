extern crate cgmath;
use std::ops::*;
use cgmath::{Vector2, MetricSpace, InnerSpace};

const MAX_SPEED: f32 = 4.0;
const MAX_STEER_FORCE: f32 = 0.5;
const DESIRED_SEPARATION: f32 = 500.0;

#[derive(Debug, PartialEq, Clone)]
pub struct Boid {
  pub position: Vector2<f32>,
  pub velocity: Vector2<f32>,
}

pub fn flock(boid: &Boid, neighbours: Vec<&Boid>, width: f32, height: f32) -> Boid {
    let avoid = avoid_walls(boid.position, width, height);
    let alignment = align(&neighbours);
    let separation = separate(&boid, &neighbours);
    let cohesion = cohede(&boid, &neighbours);

    let acceleration = Vector2 { x: 0.0, y: 0.0 }
    .add(alignment)
    .add(separation)
    .add(cohesion)
    .add(avoid);

    let mut velocity = boid.velocity.add(acceleration);
    let length = velocity.magnitude();

    if length > MAX_SPEED {
        velocity.div_assign(length / MAX_SPEED);
    }

    Boid {
        position: boid.position.add(velocity),
        velocity: velocity,
    }
}

fn avoid(position: Vector2<f32>, target: Vector2<f32>) -> Vector2<f32> {
    let mut steer = position.sub(target);
    steer.mul_assign(1.0 / position.distance2(target));
    steer
}

fn avoid_walls(position: Vector2<f32>, width: f32, height: f32) -> Vector2<f32> {
    let mut acceleration = Vector2 { x: 0.0, y: 0.0 };

    let mut target = avoid(position, Vector2 { x: 0.0, y: position.y });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    let mut target = avoid(position, Vector2 { x: width, y: position.y });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    let mut target = avoid(position, Vector2 { x: position.x, y: 0.0 });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    let mut target = avoid(position, Vector2 { x: position.x, y: height });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    acceleration
}

fn align(neighbours: &Vec<&Boid>) -> Vector2<f32> {
    let steer = neighbours.iter()
    .fold(Vector2{ x: 0.0, y: 0.0 }, |steer, ref boid| {
        steer.add(boid.velocity)
    });

    let length = neighbours.len();
    if length == 0 {
        return steer;
    }

    steer.div(length as f32 / MAX_STEER_FORCE)
}

fn separate(current: &Boid, neighbours: &Vec<&Boid>) -> Vector2<f32> {
    neighbours.iter()
    .fold(Vector2{ x: 0.0, y: 0.0 }, |steer, ref boid| {
        let distance = boid.position.distance2(current.position);

        if distance == 0.0 || distance > DESIRED_SEPARATION {
            return steer;
        }

        let diff = current
        .position
        .sub(boid.position)
        .normalize()
        .div(distance);

        steer.add(diff)
    })
}

fn cohede(current: &Boid, neighbours: &Vec<&Boid>) -> Vector2<f32> {
    let count = neighbours.len() as f32;
    if count == 0.0 {
        return Vector2{ x: 0.0, y: 0.0 };
    }

    let steer = neighbours.iter()
    .fold(Vector2{ x: 0.0, y: 0.0 }, |steer, ref boid| {
        steer.add(boid.position)
    })
    .div(neighbours.len() as f32)
    .sub(current.position);

    let length = steer.magnitude();

    if length < MAX_STEER_FORCE {
        return steer;
    }

    steer.div(length / MAX_STEER_FORCE)
}
