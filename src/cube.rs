use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use std::rc::Rc;

pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub mat_ptr: Rc<dyn Material>,
}

impl Cube {
    pub fn new(center: Point3, size: f64, m: Rc<dyn Material>) -> Cube {
        let half_size = size / 2.0;
        Cube {
            min: center - Vec3::new(half_size, half_size, half_size),
            max: center + Vec3::new(half_size, half_size, half_size),
            mat_ptr: m,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut t_near = t_min;
        let mut t_far = t_max;
        
        let mut normal = Vec3::new(0.0, 0.0, 0.0);

        // X-axis
        let inv_d = 1.0 / r.direction().x;
        let mut t0 = (self.min.x - r.origin().x) * inv_d;
        let mut t1 = (self.max.x - r.origin().x) * inv_d;
        if inv_d < 0.0 { std::mem::swap(&mut t0, &mut t1); }
        
        if t0 > t_near { 
            t_near = t0; 
            // Normal depends on which face we hit. Here if strictly greater, we updated t_near.
            // Simplified normal logic requires tracking which axis updated t_near last.
            normal = Vec3::new(-1.0, 0.0, 0.0); // Default placeholder logic, needs refinement
            if inv_d < 0.0 { normal = Vec3::new(1.0, 0.0, 0.0); } else { normal = Vec3::new(-1.0, 0.0, 0.0); }
        }
        if t1 < t_far { t_far = t1; }
        if t_near > t_far { return false; }

        // Y-axis
        let inv_d = 1.0 / r.direction().y;
        let mut t0 = (self.min.y - r.origin().y) * inv_d;
        let mut t1 = (self.max.y - r.origin().y) * inv_d;
        if inv_d < 0.0 { std::mem::swap(&mut t0, &mut t1); }

        if t0 > t_near {
            t_near = t0;
            if inv_d < 0.0 { normal = Vec3::new(0.0, 1.0, 0.0); } else { normal = Vec3::new(0.0, -1.0, 0.0); }
        }
        if t1 < t_far { t_far = t1; }
        if t_near > t_far { return false; }

        // Z-axis
        let inv_d = 1.0 / r.direction().z;
        let mut t0 = (self.min.z - r.origin().z) * inv_d;
        let mut t1 = (self.max.z - r.origin().z) * inv_d;
        if inv_d < 0.0 { std::mem::swap(&mut t0, &mut t1); }

        if t0 > t_near {
            t_near = t0;
            if inv_d < 0.0 { normal = Vec3::new(0.0, 0.0, 1.0); } else { normal = Vec3::new(0.0, 0.0, -1.0); }
        }
        if t1 < t_far { t_far = t1; }
        if t_near > t_far { return false; }

        rec.t = t_near;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, &normal);
        rec.mat_ptr = Some(self.mat_ptr.clone());

        true
    }
}
