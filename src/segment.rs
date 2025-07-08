use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Segment(pub Vector3<f64>, pub Vector3<f64>);

impl Segment {
    fn includes(&self, p: Vector3<f64>) -> bool {
        let dot = self.0.dot(&self.1);
        let in1 = self.0.dot(&p) > dot;
        let in2 = self.1.dot(&p) > dot;

        in1 && in2
    }

    pub fn intersects(&self, other: &Segment) -> bool {
        let n1 = self.0.cross(&self.1);
        let n2 = other.0.cross(&other.1);

        let v1 = n1.cross(&n2).normalize();
        let v2 = -v1;

        let v1ok = self.includes(v1) && other.includes(v1);
        let v2ok = self.includes(v2) && other.includes(v2);

        v1ok || v2ok
    }
}
