use super::Material;
use super::HitRecord;
use super::vec3::{Vec3, Ray};
use rand::{ThreadRng, Rng};

pub struct Lambertian {
    albedo: Vec3,
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{
            albedo: albedo,
        }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal{
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric {
            ref_idx: ref_idx,
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

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> (Vec3, Option<Ray>) {
        let reflected= reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere(rng) * self.fuzz);
        if scattered.direction().dot(&rec.normal) > 0f32 {
            (self.albedo, Some(scattered))
        } else {
            (self.albedo, None)
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> (Vec3, Option<Ray>) {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let reflect_prob: f32;
        let cosine: f32;
        let scattered: Ray;
        if r_in.direction().dot(&rec.normal) > 0.0 {
            outward_normal =  -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction().dot(&rec.normal) / r_in.direction().len();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction().dot(&rec.normal) / r_in.direction().len();
        }

        let reflected = reflect(r_in.direction(), rec.normal);
        if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
            reflect_prob = schlick(cosine, self.ref_idx);
            if rng.gen::<f32>() < reflect_prob {
                scattered = Ray::new(rec.p, reflected);
            } else {
                scattered = Ray::new(rec.p, refracted);
            }
        } else {
            scattered = Ray::new(rec.p, reflected);
        }
        (attenuation, Some(scattered))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(&n) * 2f32
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        Some((uv - n*dt)*ni_over_nt - n*discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
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

