use crate::{constants::TRIANGLES, triangle::Triangle};
use nalgebra::Vector3;

fn find_containing(v: &Vector3<f64>, ts: &[Triangle]) -> usize {
    ts.iter().position(|t| t.contains_point(v)).unwrap()
}

fn vec_to_hash_with_tris(v: &Vector3<f64>, ts: &[Triangle], depth: u8) -> String {
    if depth <= 0 {
        return "".into();
    }

    let index = find_containing(v, ts);
    let subdivided = ts[index].subdivide();
    let rest = vec_to_hash_with_tris(v, &subdivided, depth - 1);

    format!("{}{}", index, rest)
}

pub fn vec_to_hash(v: &Vector3<f64>, depth: u8) -> String {
    let first = find_containing(v, &TRIANGLES);
    let subdivided = TRIANGLES[first].subdivide();
    let rest = vec_to_hash_with_tris(v, &subdivided, depth - 1);

    format!("{}{}", ('a' as u8 + first as u8) as char, rest)
}

fn hash_to_tri_with_tri(hash: &str, t: &Triangle) -> Triangle {
    if hash.is_empty() {
        return *t;
    }

    let index = hash[..1].parse::<usize>().unwrap();
    let next = t.subdivide()[index];

    hash_to_tri_with_tri(&hash[1..], &next)
}

pub fn hash_to_vec(hash: &str) -> Vector3<f64> {
    let index = (hash.chars().next().unwrap() as u8 - 'a' as u8) as usize;
    let next = TRIANGLES[index];

    hash_to_tri_with_tri(&hash[1..], &next).center()
}

pub fn hash_to_tri(hash: &str) -> Triangle {
    let index = (hash.chars().next().unwrap() as u8 - 'a' as u8) as usize;
    let next = TRIANGLES[index];

    hash_to_tri_with_tri(&hash[1..], &next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_hash_to_vec() {
        let mut rng = rand::rng();

        for _ in 0..100 {
            let mut hash = (('a' as u8 + rng.random_range(0..20)) as char).to_string();

            for _ in 0..20 {
                hash += &rng.random_range(0..4).to_string();
            }

            let vec = hash_to_vec(&hash);
            let hash2 = vec_to_hash(&vec, 21);

            assert_eq!(hash, hash2);
        }
    }
}
