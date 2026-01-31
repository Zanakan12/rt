use rt::camera::Camera;
use rt::hittable::{Hittable, HitRecord};
use rt::hittable_list::HittableList;
use rt::material::{Lambertian};
use rt::plane::Plane;
use rt::cube::Cube;
use rt::ray::Ray; // Added import
use rt::vec3::{Color, Point3, Vec3};
use std::io::{self, Write};
use std::rc::Rc;
use rand::Rng;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 { return Color::new(0.0, 0.0, 0.0); }
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
    // Low brightness background for "lower brightness" requirement?
    // Or just dark material? The requirement says "cube with lower brightness than in the sphere image".
    // I will reduce the skybox factor or the material albedo.
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    // Darker sky
    (Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t) * 0.2
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let mut world = HittableList::new();
    let mat_plane = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mat_cube = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.2))); // Dark cube

    world.add(Box::new(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), mat_plane.clone())));
    world.add(Box::new(Cube::new(Point3::new(0.0, 0.0, -1.0), 1.0, mat_cube.clone())));

    let lookfrom = Point3::new(3.0, 2.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(lookfrom, lookat, vup, 45.0, aspect_ratio);

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
}

fn write_color(pixel_color: Color, samples: i32) {
    let scale = 1.0 / samples as f64;
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();
    println!("{} {} {}", (256.0 * r.clamp(0.0, 0.999)) as i32, (256.0 * g.clamp(0.0, 0.999)) as i32, (256.0 * b.clamp(0.0, 0.999)) as i32);
}
