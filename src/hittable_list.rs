use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::vec::Vec;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = HitRecord {
                    p: temp_rec.p,
                    normal: temp_rec.normal,
                    t: temp_rec.t,
                    front_face: temp_rec.front_face,
                    mat_ptr: temp_rec.mat_ptr.clone(),
                };
            }
        }

        hit_anything
    }
}
