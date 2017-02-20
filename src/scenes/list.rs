use super::super::vec3::{Ray};
use super::super::HitRecord;
use super::super::Hitable;

pub struct List<'a> {
    pub list: Vec<&'a Hitable>,
}

impl <'a> List<'a> {
    pub fn new() -> List<'a> {
        List{list: Vec::new()}
    }

    pub fn add(&mut self, scene: &'a Hitable) {
        self.list.push(scene);
    }
} 

impl <'a> Hitable for List<'a> {
    fn hit(&self, tmin: f32, tmax: f32, r: &Ray) -> Option<HitRecord> {
        let mut ret: Option<HitRecord> = None;
        let mut closest_so_far: f32 = tmax;
        for hitable in self.list.iter() {
            if let Some(h) = hitable.hit(tmin, closest_so_far, r) {
                closest_so_far = h.t;
                ret = Some(h);
            }
        }
        ret 
    }
}
