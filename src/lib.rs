extern crate rand;
pub mod vec3;
pub mod scenes;
pub mod materials;

use vec3::Vec3;
use vec3::Ray;
use rand::{ThreadRng};

#[cfg(test)]
mod tests {
    use super::vec3::Vec3;

    #[test]
    fn test_make_unit_vector() {
        let mut v = Vec3::new(4.0, 3.0, 4.0);
        assert!(v.len() != 1.0);
        v.make_unit_vector();
        assert!(v.len() == 1.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1f32, 0f32, 0f32);
        let v2 = Vec3::new(0f32, 1f32, 0f32);
        assert!(v1.cross(&v2) == Vec3::new(0f32, 0f32, 1f32));
    }

    #[test]
    fn test_arith() {
        let v1 = Vec3::new(1f32, 0f32, 0f32);
        let v2 = Vec3::new(0f32, 1f32, 0f32);
        assert!(v1 + v2 == Vec3::new(1f32, 1f32, 0f32));
    }
}

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl <'a> HitRecord <'a> {
    fn new(t: f32, p: Vec3, normal: Vec3, material: &'a Material) -> HitRecord<'a> {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }
}

pub trait Hitable {
    fn hit(&self, tmin: f32, tmax: f32, r: &Ray) -> Option<HitRecord>;
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> (Vec3, Option<Ray>);
}
