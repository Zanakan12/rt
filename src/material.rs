use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        Metal { albedo: a, fuzz: if f < 1.0 { f } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

// Utility functions for random generation (naive implementation for now)
// Since rand crate might not be available or I should check Cargo.toml, I will use a simple pseudo-random or request rand dependency.
// For now, let's assume we can use `rand` crate. I'll check Cargo.toml first.
// Actually, I'll allow myself to add `rand` to Cargo.toml.

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * Vec3::dot(&v, &n) * 2.0
}

// Placeholder for random functions, will be moved/implemented properly.
// I will implement a custom simple random number generator if I don't want to use dependencies, 
// strictly speaking the user environment might not have internet access? 
// The environment usually allows crate downloads. I will check.
// For safety, I'll use a simple LCG or Xorshift for now in a separate utility module or inside vec3?
// Actually, `random_unit_vector` needs to be in `vec3`.

use crate::vec3::random_unit_vector;
use crate::vec3::random_in_unit_sphere;
