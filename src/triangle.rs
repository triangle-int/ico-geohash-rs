use glam::Vec3;

use crate::segment::Segment;

#[derive(Debug, Clone, Copy)]
pub struct Triangle(pub [Vec3; 3]);

impl Triangle {
    pub fn segments(&self) -> [Segment; 3] {
        [
            Segment(self.0[0], self.0[1]),
            Segment(self.0[1], self.0[2]),
            Segment(self.0[2], self.0[0]),
        ]
    }

    pub fn contains_point(&self, p: Vec3) -> bool {
        for i in 0..3 {
            let n = self.0[i].cross(self.0[(i + 1) % 3]);

            if n.dot(p) > 0.0 {
                return false;
            }
        }

        true
    }

    pub fn subdivide(&self) -> [Triangle; 4] {
        let middles = [
            (self.0[0] + self.0[1]).normalize(),
            (self.0[1] + self.0[2]).normalize(),
            (self.0[2] + self.0[0]).normalize(),
        ];

        [
            Triangle([self.0[0], middles[0], middles[2]]),
            Triangle([self.0[1], middles[1], middles[0]]),
            Triangle([self.0[2], middles[2], middles[1]]),
            Triangle([middles[0], middles[1], middles[2]]),
        ]
    }

    pub fn center(&self) -> Vec3 {
        (self.0[0] + self.0[1] + self.0[2]).normalize()
    }
}
