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
use cgmath::{Vector2, MetricSpace};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

const SIZE: f64 = 600.0;

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
    let acceleration = Vector2 { x: 0.0, y: 0.0 };

    let target = Vector2 { x: 0.0, y: position.y };
    let mut target = avoid(position, target);
    target.mul_assign(5.0);
    let acceleration = acceleration.add(target);

    let target = Vector2 { x: SIZE, y: position.y };
    let mut target = avoid(position, target);
    target.mul_assign(5.0);
    let acceleration = acceleration.add(target);

    let target = Vector2 { x: position.x, y: 0.0 };
    let mut target = avoid(position, target);
    target.mul_assign(5.0);
    let acceleration = acceleration.add(target);

    let target = Vector2 { x: position.x, y: SIZE };
    let mut target = avoid(position, target);
    target.mul_assign(5.0);
    let acceleration = acceleration.add(target);

    return acceleration;
}

fn flock(boid: &Boid, _boids: &Vec<Boid>) -> Boid {
    let acceleration = avoid_walls(boid.position);
    let velocity = boid.velocity.add(acceleration);

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
            position: Vector2 { x: i as f64, y: i as f64},
            velocity: Vector2 { x: 1.0, y: 2.0 },
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

        context.set_fill_style_color("#dddddd");
        context.fill_rect(0.0, 0.0, SIZE, SIZE);
        context.set_fill_style_color("#ff0000");

        for boid in boids.iter() {
            context.fill_rect(boid.position.x, boid.position.y, 5.0, 5.0);
        }

        window().request_animation_frame(move |_| {
            main_loop(context, canvas, boids);
        });
    }

    main_loop(context, canvas, boids);

    stdweb::event_loop();
}
