extern crate stdweb;
extern crate cgmath;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d,
};
use stdweb::web::event::{
    ResizeEvent,
};
use std::ops::*;
use cgmath::{Vector2, MetricSpace, InnerSpace};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 700.0;

struct Boid {
  position: Vector2<f64>,
  velocity: Vector2<f64>,
}

fn avoid(position: Vector2<f64>, target: Vector2<f64>) -> Vector2<f64> {
    let mut steer = position.sub(target);
    steer.mul_assign(1.0 / position.distance2(target));
    steer
}

fn avoid_walls(position: Vector2<f64>) -> Vector2<f64> {
    let mut acceleration = Vector2 { x: 0.0, y: 0.0 };

    let mut target = avoid(position, Vector2 { x: 0.0, y: position.y });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    // let target = Vector2 { x: WIDTH, y: position.y };
    let mut target = avoid(position, Vector2 { x: WIDTH, y: position.y });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    let mut target = avoid(position, Vector2 { x: position.x, y: 0.0 });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    let mut target = avoid(position, Vector2 { x: position.x, y: HEIGHT });
    target.mul_assign(25.0);
    acceleration.add_assign(target);

    acceleration
}

const MAX_SPEED: f64 = 2.0;
const MAX_STEER_FORCE: f64 = 0.02;
const NEIGHBOUR_DISTANCE: f64 = 200.0;
const DESIRED_SEPARATION: f64 = 25.0;

fn align(neighbours: &Vec<&Boid>) -> Vector2<f64> {
    let steer = neighbours.iter()
    .fold(Vector2{ x: 0.0, y: 0.0 }, |steer, &boid| {
        steer.add(boid.velocity)
    });

    let length = neighbours.len();
    if length == 0 {
        return steer;
    }

    steer.div(length as f64 / MAX_STEER_FORCE)
}

fn separate(current: &Boid, neighbours: &Vec<&Boid>) -> Vector2<f64> {
    let mut steer = Vector2{ x: 0.0, y: 0.0 };

    for boid in neighbours.iter() {
        let distance = boid.position.distance2(current.position);

        if distance == 0.0 || distance > DESIRED_SEPARATION {
            continue;
        }

        let diff = current
        .position
        .sub(boid.position)
        .normalize()
        .div(distance);

        steer.add_assign(diff);
    }
    steer
}

fn cohede(current: &Boid, neighbours: &Vec<&Boid>) -> Vector2<f64> {
    let count = neighbours.len() as f64;
    let steer = neighbours.iter()
    .fold(Vector2{ x: 0.0, y: 0.0 }, |steer, &boid| {
        steer.add(boid.position)
    })
    .div(count)
    .sub(current.position);
    let length = steer.magnitude();

    if length < MAX_STEER_FORCE {
        return steer;
    }

    steer.div(length / MAX_STEER_FORCE)
}

fn flock(boid: &Boid, boids: &Vec<Boid>) -> Boid {
    let avoid = avoid_walls(boid.position);
    let position = boid.position;

    let neighbours = boids.iter().filter(|&boid| {
        let distance = boid.position.distance2(position);
        distance > 0.0 && distance < NEIGHBOUR_DISTANCE
    }).collect::<Vec<&Boid>>();

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

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector("#canvas").unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    // Add some boids
    let mut boids = Vec::new();

    for i in 0..1000 {
        boids.push(Boid {
            position: Vector2 { x: 400.0 + i as f64 * 0.1, y: 300.0 },
            velocity: Vector2 { x: 0.1, y: 0.2 },
        })
    }

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);
    context.set_fill_style_color("#ff0000");

    window().add_event_listener(enclose!((canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    fn main_loop(context: CanvasRenderingContext2d, canvas: CanvasElement, boids: Vec<Boid>) {
        let boids = boids.iter().map(|boid| {
            return flock(boid, &boids);
        }).collect::<Vec<Boid>>();

        let width = canvas.width();
        let height = canvas.height();

        context.clear_rect(0.0, 0.0, width as f64, height as f64);

        context.set_fill_style_color("#000000");
        context.fill_rect(0.0, 0.0, WIDTH, HEIGHT);
        context.set_fill_style_color("#00ffff");

        for boid in boids.iter() {
            context.fill_rect(boid.position.x, boid.position.y, 2.0, 2.0);
        }

        window().request_animation_frame(move |_| {
            main_loop(context, canvas, boids);
        });
    }

    main_loop(context, canvas, boids);

    stdweb::event_loop();
}
