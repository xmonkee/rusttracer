use super::super::vec3::{Vec3, Ray};
use super::super::HitRecord;
use super::super::Hitable;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere{
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
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
                return Some(HitRecord{
                    t: temp, 
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            } 

            let temp = (-b + discriminant.sqrt()) / (2f32 * a);
            if temp > tmin &&  temp < tmax {
                let p = r.point_at_parameter(temp);  
                return Some(HitRecord{
                    t: temp, 
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            }
        }
        return None;
    }

}

