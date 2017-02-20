use super::super::vec3::{Vec3, Ray};
use super::super::HitRecord;
use super::super::Hitable;
use super::super::Material;

pub struct Sphere<'a>{
    center: Vec3,
    radius: f32,
    material: &'a Material,
}

impl <'a> Sphere<'a>{
    pub fn new<M: Material + 'static>(center: Vec3, radius: f32, material: &'a M) -> Sphere<'a>{
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl <'a> Hitable for Sphere <'a>{
    fn hit(&self, tmin: f32, tmax: f32, r: &Ray) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction()) * 2.0;
        let c = oc.dot(&oc) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0f32 {

            let temp = (-b - discriminant.sqrt()) / (2f32 * a);
            if temp > tmin &&  temp < tmax {
                let p = r.point_at_parameter(temp);  
                let h = HitRecord::new(temp, p, (p - self.center) / self.radius, self.material,);
                return Some(h);
            } 

            let temp = (-b + discriminant.sqrt()) / (2f32 * a);
            if temp > tmin &&  temp < tmax {
                let p = r.point_at_parameter(temp);
                let h = HitRecord::new(temp, p, (p - self.center) / self.radius, self.material,);
                return Some(h);
            }
        }
        return None;
    }

}

