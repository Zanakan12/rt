# Ray Tracer Documentation

## Introduction
This is a simple Ray Tracer written in Rust. It renders 3D scenes with spheres, cubes, planes, and cylinders, supporting diffuse and metallic materials with shadows and reflections.

## Features
- **Shapes**: Sphere, Cube, Plane, Cylinder (capped by height but without disks for now, effectively open pipe if seen from top, but vertical sides work).
- **Materials**: 
  - `Lambertian`: Diffuse (matte) material.
  - `Metal`: Reflective material with adjustable fuzziness.
- **Lighting**: Global illumination via path tracing (Sky light + bounce light).
- **Camera**: Movable camera with FOV control.
- **Anti-aliasing**: Multi-sampling per pixel.

## Usage

### Prerequisites
- [Rust & Cargo installed](https://www.rust-lang.org/tools/install)

### Building and Running
To render the default scene:
```sh
cargo run --release > output.ppm
```
This will generate an `output.ppm` file which can be opened with standard image viewers (e.g., GIMP, Photoshop, or online PPM viewers).
Note: `--release` is highly recommended as ray tracing is computationally expensive.

### Modifying the Scene
The scene is defined in `src/main.rs`. You can edit this file to change objects, materials, or camera settings.

#### Creating Objects
Objects are added to the `world` variable.
```rust
// Sphere: Center (0,0,-1), Radius 0.5, Material
world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone())));

// Cube: Center, Side Length, Material
world.add(Box::new(Cube::new(Point3::new(-1.0, 0.0, -1.0), 1.0, material_left.clone())));

// Cylinder: Center, Radius, Height, Material
world.add(Box::new(Cylinder::new(Point3::new(1.0, 0.0, -1.0), 0.5, 1.0, material_right.clone())));

// Plane: Point on plane, Normal vector, Material
world.add(Box::new(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), material_ground.clone())));
```

#### Changing Materials
Create materials using `Rc<dyn Material>`:
```rust
// Diffuse (Color R, G, B)
let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));

// Metal (Color, Fuzziness [0.0-1.0])
let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
```

#### Moving the Camera
In `src/main.rs`, update the camera initialization:
```rust
let lookfrom = Point3::new(3.0, 3.0, 2.0); // Camera position
let lookat = Point3::new(0.0, 0.0, -1.0);  // Target point
let vup = Vec3::new(0.0, 1.0, 0.0);        // Up vector (HEAD rotation)
let vfov = 20.0;                           // Vertical Field of View in degrees

let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio);
```

#### Changing Resolution
Update `image_width` in `main.rs`. `image_height` is calculated automatically based on aspect ratio.
```rust
let image_width = 800; // Examples: 1920, 800, 400
```
