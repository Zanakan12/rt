use rt::camera::Camera;
use rt::cube::Cube;
use rt::cylinder::Cylinder;
use rt::hittable::{Hittable, HitRecord};
use rt::hittable_list::HittableList;
use rt::material::{Lambertian, Metal, Material};
use rt::plane::Plane;
use rt::ray::Ray;
use rt::sphere::Sphere;
use rt::vec3::{Color, Point3, Vec3};

use std::io::{self, Write};
use std::rc::Rc;
use rand::Rng;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        
        if let Some(mat) = &rec.mat_ptr {
             if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // Ground Plane
    world.add(Box::new(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), material_ground.clone())));

    // Objects
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone())));
    world.add(Box::new(Cube::new(Point3::new(-1.0, 0.0, -1.0), 1.0, material_left.clone())));
    world.add(Box::new(Cylinder::new(Point3::new(1.0, 0.0, -1.0), 0.5, 1.0, material_right.clone())));

    // Camera
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let _dist_to_focus = (lookfrom - lookat).length();
    let _aperture = 2.0;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut rng = rand::rng();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.random::<f64>()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}

fn write_color(pixel_color: Color, samples: i32) {
    let scale = 1.0 / samples as f64;
    // Gamma correction 2.0
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32
    );
}
