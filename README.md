# Birbs

A [boids](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/boids.html) implementation in Rust for WebAssembly.

## Why?

Well i'm learning Rust and the boids simulation facinates me.

## TODO

* It uses a quadtree to make looking up neighbours really quick but this results in undesirable clustering.
* Give the boids some form and make their direction clear (at the moment they're just 2x2px squares)
* Add some obstacles

# Getting it running

At the time of development you'd need a nightly build of Rust (I was on 1.31.0 — if you find the project is no longer working on later version let me know!).

You'll also need cargo-web:

```bash
cargo install cargo-web

```

With cargo web and any dependencies it demands you can start the project with

```bash
cargo web start --target=wasm32-unknown-unknown
```

This will build the project and serve it up over a local web server

**Tests**

I'm still new to Rust but there are a few tests

```bash
cargo test
```

# Demo

It's not hosted anywhere currently but here is a gif with 5000 particles 😉

![](https://user-images.githubusercontent.com/122096/46473322-ac745000-c7d7-11e8-85c3-883dff791b9d.gif)