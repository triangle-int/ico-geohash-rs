use crate::{constants::TRIANGLES, triangle::Triangle};

/*
const AREA_RATIO: f64 = 10.0;

fn find_optimal_level(area: f64) -> u8 {
    let s_1 = 4.0 * PI / 20.0;
    let s_n = area / AREA_RATIO;

    let ratio = s_1 / s_n;
    let l = ratio.log(4.0) + 1.0;

    l.round() as u8
}
*/

pub trait Coverable {
    fn intersects(&self, tri: &Triangle) -> bool;

    fn find_coverage(&self, depth: u8) -> Vec<String>
    where
        Self: Sized,
    {
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

    fn find_tri_coverage(&self, depth: u8) -> Vec<Triangle>
    where
        Self: Sized,
    {
        let mut result = Vec::new();

        for i in 0..TRIANGLES.len() {
            find_tri_coverage_with_tri(self, &TRIANGLES[i], depth, 1, &mut result);
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

    if acc.len() >= depth as usize {
        res.push(acc);
        return;
    }

    let tris = tri.subdivide();

    for i in 0..tris.len() {
        find_coverage_with_tri(shape, &tris[i], depth, acc.clone() + &i.to_string(), res);
    }
}

fn find_tri_coverage_with_tri(
    shape: &impl Coverable,
    tri: &Triangle,
    depth: u8,
    curr_depth: u8,
    res: &mut Vec<Triangle>,
) {
    if !shape.intersects(&tri) {
        return;
    }

    if curr_depth >= depth {
        res.push(*tri);
        return;
    }

    let tris = tri.subdivide();

    for i in 0..tris.len() {
        find_tri_coverage_with_tri(shape, &tris[i], depth, curr_depth + 1, res);
    }
}
