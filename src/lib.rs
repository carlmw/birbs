extern crate wasm_bindgen;
extern crate cgmath;
extern crate ntree;

use wasm_bindgen::prelude::*;
use cgmath::{Vector2};
mod boid;
use boid::{Boid,flock};
mod tree;
use tree::{QuadTreeRegion};
use ntree::{NTree};

const WIDTH: f32 = 1910.0;
const HEIGHT: f32 = 1080.0;
const BUCKET_SIZE: u8 = 50;
const WORLD: QuadTreeRegion = QuadTreeRegion { x: 0.0, y: 0.0, width: WIDTH, height: HEIGHT };
#[wasm_bindgen]
pub struct BoidManager {
    boids: NTree<QuadTreeRegion, Boid>,
    all: Vec<f32>,
    count: u32,
}

#[wasm_bindgen]
impl BoidManager {
    #[wasm_bindgen(constructor)]
    pub fn new(count: u32) -> BoidManager {
        let mut boids = NTree::new(QuadTreeRegion::square(0.0, 0.0, WIDTH), BUCKET_SIZE);
        let mut all = Vec::with_capacity(count as usize);

        for i in 0..count {
            let boid = Boid {
                position: Vector2 { x: 100.0 + i as f32 * 0.1, y: 100.0 },
                velocity: Vector2 { x: 0.0, y: 0.0 },
            };
            all.push(boid.position.x);
            all.push(boid.position.y);
            all.push(boid.velocity.x);
            all.push(boid.velocity.y);
            boids.insert(boid);
        }

        BoidManager { boids: boids, all: all, count: count }
    }

    pub fn flock(&mut self) {
        let mut new_boids = NTree::new(QuadTreeRegion::square(0.0, 0.0, WIDTH), BUCKET_SIZE);
        {
            let all = self.boids.range_query(&WORLD);
            for (i, boid) in all.enumerate() {
                let boid = match self.boids.nearby(boid) {
                    Some(neighbours) => flock(boid, neighbours.iter().collect::<Vec<&Boid>>(), WIDTH, HEIGHT),
                    None => boid.clone(),
                };
                self.all[i * 4] = boid.position.x;
                self.all[i * 4 + 1] = boid.position.y;
                self.all[i * 4 + 2] = boid.velocity.x;
                self.all[i * 4 + 3] = boid.velocity.y;
                new_boids.insert(boid);
            }
        }
        self.boids = new_boids;
    }

    pub fn items(&self) -> *const f32 {
        self.all.as_ptr()
    }

    pub fn length(&self) -> u32 {
        self.count * 4
    }
}
