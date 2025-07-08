use crate::{constants::TRIANGLES, triangle::Triangle};
use std::f64::consts::PI;

const AREA_RATIO: f64 = 10.0;

fn find_optimal_level(area: f64) -> u8 {
    let s_1 = 4.0 * PI / 20.0;
    let s_n = area / AREA_RATIO;

    let ratio = s_1 / s_n;
    let l = ratio.log(4.0) + 1.0;

    l.round() as u8
}

pub trait Coverable {
    fn intersects(&self, tri: &Triangle) -> bool;
    fn contains(&self, tri: &Triangle) -> bool;
    fn area(&self) -> f64;

    fn find_coverage(&self) -> Vec<String>
    where
        Self: Sized,
    {
        let depth = find_optimal_level(self.area());
        let mut result = Vec::new();

        for i in 0..TRIANGLES.len() {
            find_coverage_with_tri(
                self,
                &TRIANGLES[i],
                depth,
                (('a' as u8 + i as u8) as char).to_string(),
                &mut result,
            );
        }

        result
    }
}

fn find_coverage_with_tri(
    shape: &impl Coverable,
    tri: &Triangle,
    depth: u8,
    acc: String,
    res: &mut Vec<String>,
) {
    if !shape.intersects(&tri) {
        return;
    }

    if shape.contains(&tri) || acc.len() >= depth as usize {
        res.push(acc);
        return;
    }

    let tris = tri.subdivide();

    for i in 0..tris.len() {
        find_coverage_with_tri(shape, &tris[i], depth, acc.clone() + &i.to_string(), res);
    }
}
