use super::super::vec3::{Ray};
use super::super::Hitable;
use super::super::HitRecord;

pub struct Blank {
}

impl Blank {
    pub fn new() -> Blank {
        Blank{}
    }
}

impl Hitable for Blank {
    fn hit(&self, tmin: f32, tmax: f32, r: &Ray) -> Option<HitRecord> {
        None
    }
}
