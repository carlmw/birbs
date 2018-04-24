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
use cgmath::{Vector2};
mod boid;
use boid::{Boid,flock};

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
            return flock(boid, &boids, WIDTH, HEIGHT);
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
