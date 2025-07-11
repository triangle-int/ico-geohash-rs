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

fn find_tris_of_level_with_tri(
    triangle: &Triangle,
    acc: String,
    depth: u8,
    out: &mut Vec<(String, Triangle)>,
) {
    if acc.len() >= depth as usize {
        out.push((acc, *triangle));
        return;
    }

    let next = triangle.subdivide();

    for i in 0..next.len() {
        find_tris_of_level_with_tri(&next[i], format!("{}{}", acc, i), depth, out);
    }
}

pub fn find_tris_of_level(depth: u8) -> Vec<(String, Triangle)> {
    let mut res = Vec::new();

    for i in 0..TRIANGLES.len() {
        find_tris_of_level_with_tri(
            &TRIANGLES[i],
            (('a' as u8 + i as u8) as char).into(),
            depth,
            &mut res,
        );
    }

    res
}

pub fn find_neighbours(hash: &str) -> [String; 3] {
    let triangle = hash_to_tri(hash);
    let center = triangle.center();

    let mut res = <[String; 3]>::default();

    for i in 0..3 {
        let v1 = triangle.0[i];
        let v2 = triangle.0[(i + 1) % 3];

        let n = v1.cross(&v2).normalize();
        let mirrored = center - 2.0 * n.dot(&center) * n;

        res[i] = vec_to_hash(&mirrored, hash.len() as u8);
    }

    res
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

    #[test]
    fn test_find_tris_of_level() {
        let tris = find_tris_of_level(3);
        assert_eq!(tris.len(), 20 * 4 * 4);

        for (hash, _) in &tris {
            assert_eq!(hash.len(), 3);
        }
    }

    #[test]
    fn test_find_neighbours() {
        let hash = "b11111333";
        let neighbours = find_neighbours(&hash);

        assert!(neighbours.contains(&"b11111331".into()));
        assert!(neighbours.contains(&"b11111332".into()));
        assert!(neighbours.contains(&"b11111330".into()));
    }
}
