use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use macroquad::prelude::*;
use macroquad::{
    color::{Color, WHITE},
    math::Vec2,
    shapes::draw_line,
};
use nalgebra::Point2;
use parry2d::math::Real;
use parry2d::shape::TriMesh;

/// As this file is used as a module from other examples,
/// rustc warns about dead code:
/// - `main()` is needed for this file to be included in examples
/// - For other functions, they may be "dead code" for an example, but not for others.
#[allow(dead_code)]
fn main() {
    println!(
        "This module contains helper functions to use macroquad,
    isolated from the rest of the examples for the sake of simplicity."
    );
}

#[allow(dead_code)]
pub fn mquad_from_na(a: Point2<Real>) -> Vec2 {
    Vec2::new(a.x, a.y)
}

#[allow(dead_code)]
pub fn na_from_mquad(a: Vec2) -> Point2<Real> {
    Point2::new(a.x, a.y)
}

#[allow(dead_code)]
pub fn draw_polyline(polygon: Vec<(Vec2, Vec2)>, color: Color) {
    for i in 0..polygon.len() {
        let a = polygon[i].0;
        let b = polygon[i].1;
        draw_line_2d(a, b, color);
    }
}

#[allow(dead_code)]
pub fn easy_draw_text(text: &str) {
    macroquad::text::draw_text(text, 10.0, 48.0 + 18.0, 30.0, WHITE);
}

#[allow(dead_code)]
pub fn lissajous_2d(t: f32) -> Vec2 {
    // Some hardcoded parameters to have a pleasing lissajous trajectory.
    let (a, b, delta_x, delta_y) = (3.0, 2.0, FRAC_PI_2, FRAC_PI_4);

    let x = (a * t + delta_x).sin();
    let y = (b * t + delta_y).sin();
    Vec2::new(x, y) * 0.75f32
}

#[allow(dead_code)]
pub fn draw_line_2d(a: Vec2, b: Vec2, color: Color) {
    draw_line(a.x, a.y, b.x, b.y, 2f32, color);
}

#[allow(dead_code)]
pub fn draw_trimesh2(trimesh: &TriMesh, offset: Vec2) {
    let vertices = trimesh.vertices();
    for v in trimesh.indices() {
        let v0 = mquad_from_na(vertices[v[0] as usize]) + offset;
        let v1 = mquad_from_na(vertices[v[1] as usize]) + offset;
        let v2 = mquad_from_na(vertices[v[2] as usize]) + offset;

        draw_line(v0.x, v0.y, v1.x, v1.y, 2f32, WHITE);
        draw_line(v0.x, v0.y, v2.x, v2.y, 2f32, WHITE);
        draw_line(v2.x, v2.y, v1.x, v1.y, 2f32, WHITE);
    }
}

#[allow(dead_code)]
pub fn draw_polygon(polygon: &[Point2<f32>], scale: f32, shift: Point2<f32>, color: Color) {
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        draw_line(
            a.x * scale + shift.x,
            a.y * scale + shift.y,
            b.x * scale + shift.x,
            b.y * scale + shift.y,
            2.0,
            color,
        );
    }
}

#[allow(dead_code)]
pub fn draw_point(point: Point2<f32>, scale: f32, shift: Point2<f32>, color: Color) {
    let edge_len = 0.15;
    draw_line(
        (point.x - edge_len) * scale + shift.x,
        point.y * scale + shift.y,
        (point.x + edge_len) * scale + shift.x,
        point.y * scale + shift.y,
        2.0,
        color,
    );
    draw_line(
        point.x * scale + shift.x,
        (point.y - edge_len) * scale + shift.y,
        point.x * scale + shift.x,
        (point.y + edge_len) * scale + shift.y,
        2.0,
        color,
    );
}