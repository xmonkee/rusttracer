use super::super::vec3::{Ray};
use super::super::HitRecord;
use super::super::Hitable;

pub struct List<'a, H: Hitable + 'a> {
    pub list: Vec<&'a H>,
}

impl <'a, H: Hitable> List<'a, H> {
    pub fn new() -> List<'a, H> {
        List{list: Vec::new()}
    }

    pub fn add(&mut self, scene: &'a H) {
        self.list.push(scene);
    }
} 

impl <'a, H: Hitable + 'a> Hitable for List<'a, H> {
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
