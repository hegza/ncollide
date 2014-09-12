extern crate nalgebra;
extern crate "ncollide3df32" as ncollide;

use std::rand;
use nalgebra::na::Vec3;
use ncollide::procedural;

fn main() {
    let mut points = Vec::new();
    for _ in range(0u, 100000) {
        points.push(rand::random::<Vec3<f32>>() * 2.0f32);
    }

    let _ = procedural::convex_hull3d(points.as_slice());
}
