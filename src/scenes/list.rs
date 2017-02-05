use super::super::vec3::{Ray};
use super::super::HitRecord;
use super::super::Hitable;

pub struct List {
    pub list: Vec<Box<Hitable>>,
}

impl List {
    pub fn new() -> List {
        List{list: Vec::new()}
    }

    pub fn add<H: Hitable + 'static>(&mut self, scene: H) {
        self.list.push(Box::new(scene));
    }
} 

impl Hitable for List {
    fn hit(&self, tmin: f32, tmax: f32, r: &Ray) -> Option<HitRecord>{
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far: f32 = tmax;
        for hitable in self.list.iter() {
            if let Some(t_hit_record) = hitable.hit(tmin, closest_so_far, r) {
                closest_so_far = t_hit_record.t;
                hit_record = Some(t_hit_record);
            }
        }
        hit_record
    }
}
