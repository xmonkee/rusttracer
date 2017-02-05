pub mod vec3;
pub mod scenes;

use vec3::Vec3;
use vec3::Ray;

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

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, tmin: f32, tmax: f32, r: &Ray) -> Option<HitRecord>;
}
