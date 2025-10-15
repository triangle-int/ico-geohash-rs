use crate::triangle::Triangle;
use nalgebra::Vector3;

// 1 / sqrt(1 + phi^2)
const MAG: f64 = 0.5257311121191336;
const PHI: f64 = 1.618033988749895;
const X: f64 = 1.0 * MAG;
const Z: f64 = PHI * MAG;
const N: f64 = 0.0;

const VERTS: [Vector3<f64>; 12] = [
    Vector3::new(-X, N, Z),
    Vector3::new(X, N, Z),
    Vector3::new(-X, N, -Z),
    Vector3::new(X, N, -Z),
    Vector3::new(N, Z, X),
    Vector3::new(N, Z, -X),
    Vector3::new(N, -Z, X),
    Vector3::new(N, -Z, -X),
    Vector3::new(Z, X, N),
    Vector3::new(-Z, X, N),
    Vector3::new(Z, -X, N),
    Vector3::new(-Z, -X, N),
];

pub const TRIANGLES: [Triangle; 20] = [
    Triangle([VERTS[0], VERTS[4], VERTS[1]]),
    Triangle([VERTS[0], VERTS[9], VERTS[4]]),
    Triangle([VERTS[9], VERTS[5], VERTS[4]]),
    Triangle([VERTS[4], VERTS[5], VERTS[8]]),
    Triangle([VERTS[4], VERTS[8], VERTS[1]]),
    //
    Triangle([VERTS[8], VERTS[10], VERTS[1]]),
    Triangle([VERTS[8], VERTS[3], VERTS[10]]),
    Triangle([VERTS[5], VERTS[3], VERTS[8]]),
    Triangle([VERTS[5], VERTS[2], VERTS[3]]),
    Triangle([VERTS[2], VERTS[7], VERTS[3]]),
    //
    Triangle([VERTS[7], VERTS[10], VERTS[3]]),
    Triangle([VERTS[7], VERTS[6], VERTS[10]]),
    Triangle([VERTS[7], VERTS[11], VERTS[6]]),
    Triangle([VERTS[11], VERTS[0], VERTS[6]]),
    Triangle([VERTS[0], VERTS[1], VERTS[6]]),
    //
    Triangle([VERTS[6], VERTS[1], VERTS[10]]),
    Triangle([VERTS[9], VERTS[0], VERTS[11]]),
    Triangle([VERTS[9], VERTS[11], VERTS[2]]),
    Triangle([VERTS[9], VERTS[2], VERTS[5]]),
    Triangle([VERTS[7], VERTS[2], VERTS[11]]),
];
