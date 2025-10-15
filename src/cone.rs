use crate::{coverage::Coverable, segment::Segment, triangle::Triangle};
use nalgebra::Vector3;

const INTERSECTION_ITERS: u8 = 32;

#[derive(Debug, Clone, Copy)]
pub struct Cone {
    pub center: Vector3<f64>,
    pub angle: f64,
}

impl Cone {
    fn contains_point(&self, p: &Vector3<f64>) -> bool {
        p.angle(&self.center) < self.angle
    }

    fn intersects_segment(&self, s: &Segment) -> bool {
        let mut s = *s;

        for _ in 0..INTERSECTION_ITERS {
            if self.contains_point(&s.0) || self.contains_point(&s.1) {
                return true;
            }

            let fst = (s.0 + (s.1 - s.0) / 3.0).normalize();
            let snd = (s.0 + (s.1 - s.0) * 2.0 / 3.0).normalize();

            if self.center.dot(&fst) < self.center.dot(&snd) {
                s.0 = fst
            } else {
                s.1 = snd
            }
        }

        false
    }
}

impl Coverable for Cone {
    fn intersects(&self, tri: &Triangle) -> bool {
        tri.contains_point(&self.center)
            || tri.0.iter().any(|p| self.contains_point(p))
            || tri.segments().iter().any(|s| self.intersects_segment(s))
    }
}
