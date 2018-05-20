extern crate stdweb;
extern crate cgmath;
extern crate ntree;
extern crate random_color;
extern crate rand;

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
mod tree;
use tree::{QuadTreeRegion};
use ntree::{NTree};
use random_color::{Color, Luminosity, RandomColor};
use rand::{OsRng, RngCore};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

const WIDTH: f64 = 1910.0;
const HEIGHT: f64 = 1080.0;
const BUCKET_SIZE: u8 = 50;

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector("#canvas").unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    let mut boids = NTree::new(QuadTreeRegion::square(0.0, 0.0, WIDTH), BUCKET_SIZE);
    let mut rng = OsRng::new().expect("Error opening random number generator");

    for i in 0..5000 {
        let color = RandomColor::new()
            .hue(Color::Blue) // Optional
            .luminosity(Luminosity::Light) // Optional
            .seed(rng.next_u32() as i32)
            .to_hex();

        let boid = Boid {
            position: Vector2 { x: 100.0 + i as f64 * 0.1, y: 100.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            color: color,
        };
        boids.insert(boid);
    }

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    window().add_event_listener(enclose!((canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    const WORLD: QuadTreeRegion = QuadTreeRegion { x: 0.0, y: 0.0, width: WIDTH, height: HEIGHT };

    fn main_loop(context: CanvasRenderingContext2d, canvas: CanvasElement, boids: NTree<QuadTreeRegion, Boid>) {
        let width = canvas.width();
        let height = canvas.height();

        context.clear_rect(0.0, 0.0, width as f64, height as f64);

        context.set_fill_style_color("#000000");
        context.fill_rect(0.0, 0.0, WIDTH, HEIGHT);

        let all = boids.range_query(&WORLD);
        let mut new_boids = NTree::new(QuadTreeRegion::square(0.0, 0.0, WIDTH), BUCKET_SIZE);
        for boid in all {
            let boid = match boids.nearby(boid) {
                Some(neighbours) => flock(boid, neighbours.iter().collect::<Vec<&Boid>>(), WIDTH, HEIGHT),
                None => boid.clone(),
            };
            context.set_fill_style_color(&boid.color);
            context.fill_rect(boid.position.x, boid.position.y, 2.0, 2.0);
            new_boids.insert(boid);
        }

        window().request_animation_frame(move |_| {
            main_loop(context, canvas, new_boids);
        });
    }
    main_loop(context, canvas, boids);
    stdweb::event_loop();
}
