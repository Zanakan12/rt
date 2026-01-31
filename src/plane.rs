use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use std::rc::Rc;

pub struct Plane {
    pub point: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, m: Rc<dyn Material>) -> Plane {
        Plane { point, normal, mat_ptr: m }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denominator = Vec3::dot(&self.normal, &r.direction());
        if denominator.abs() < 1e-8 {
            return false;
        }

        let t = Vec3::dot(&(self.point - r.origin()), &self.normal) / denominator;
        if t < t_min || t > t_max {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.set_face_normal(r, &self.normal);
        rec.mat_ptr = Some(self.mat_ptr.clone());

        true
    }
}
