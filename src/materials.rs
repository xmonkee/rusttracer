use super::Material;
use super::HitRecord;
use super::vec3::{Vec3, Ray};
use rand::{ThreadRng, Rng};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{
            albedo: albedo,
        }
    }

}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> (Vec3, Option<Ray>) {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(rec.p, target - rec.p);
        (self.albedo, Some(scattered))
    }
}

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut p = Vec3::new(1f32, 1f32, 1f32);

    while p.lensq() >= 1.0 {
        p = Vec3::new(
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            rng.gen::<f32>(),
        )
    }
    return p;
}
