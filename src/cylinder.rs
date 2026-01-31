use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use std::rc::Rc;

pub struct Cylinder {
    pub center: Point3,
    pub radius: f64,
    pub height: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(center: Point3, radius: f64, height: f64, m: Rc<dyn Material>) -> Cylinder {
        Cylinder { center, radius, height, mat_ptr: m }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().x * r.direction().x + r.direction().z * r.direction().z;
        let b = 2.0 * (oc.x * r.direction().x + oc.z * r.direction().z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-b - sqrtd) / (2.0 * a);
        
        // Check for side intersection
        let mut valid_hit = false;
        
        if root >= t_min && root <= t_max {
            let y = r.origin().y + root * r.direction().y;
            if y >= self.center.y - self.height / 2.0 && y <= self.center.y + self.height / 2.0 {
                valid_hit = true;
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = Vec3::new((rec.p.x - self.center.x) / self.radius, 0.0, (rec.p.z - self.center.z) / self.radius);
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = Some(self.mat_ptr.clone());
                return true;
            }
        }
        
        if !valid_hit {
             root = (-b + sqrtd) / (2.0 * a);
             if root >= t_min && root <= t_max {
                let y = r.origin().y + root * r.direction().y;
                if y >= self.center.y - self.height / 2.0 && y <= self.center.y + self.height / 2.0 {
                    valid_hit = true;
                    rec.t = root;
                    rec.p = r.at(rec.t);
                    let outward_normal = Vec3::new((rec.p.x - self.center.x) / self.radius, 0.0, (rec.p.z - self.center.z) / self.radius);
                    rec.set_face_normal(r, &outward_normal);
                    rec.mat_ptr = Some(self.mat_ptr.clone());
                    return true;
                }
             }
        }

        // Check caps intersection (optional but nicer)
        // For simplicity, just infinite cylinder part clipped or simple caps. 
        // Let's implement basics first.
        
        false
    }
}
