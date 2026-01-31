# Ray Tracer Documentation

## Introduction
This is a simple Ray Tracer written in Rust. It renders 3D scenes with spheres, cubes, planes, and cylinders, supporting diffuse and metallic materials with shadows and reflections.

## Features
- **Shapes**: Sphere, Cube, Plane, Cylinder.
- **Materials**: 
  - `Lambertian`: Diffuse (matte) material.
  - `Metal`: Reflective material with adjustable fuzziness.
- **Lighting**: Global illumination via path tracing.
- **Camera**: Movable camera with adjustable FOV, LookFrom, LookAt.
- **Anti-aliasing**: Multi-sampling per pixel.

## Usage

### Building and Running
To render the default scene:
```sh
cargo run --release > output.ppm
```

### Audit Scenarios
To check the specific scenarios requested by the audit, special binaries have been prepared:

1. **Sphere Only**:
   ```sh
   cargo run --release --bin scenario1 > scenario1.ppm
   ```

2. **Plane + Cube (Low Brightness)**:
   ```sh
   cargo run --release --bin scenario2 > scenario2.ppm
   ```

3. **All Objects**:
   ```sh
   cargo run --release --bin scenario3 > scenario3.ppm
   ```

4. **All Objects (Different Perspective)**:
   ```sh
   cargo run --release --bin scenario4 > scenario4.ppm
   ```

### Modifying the Scene
The main scene is defined in `src/main.rs`. You can edit this file to change objects, materials, or camera settings as described in the comments internally.
To change resolution, edit the `image_width` variable in `main.rs`.
To move the camera, edit the `lookfrom` and `lookat` variables in `main.rs`.

### Windows Users
Since PPM files are not natively supported on Windows, a Python script is included to convert them to BMP:
```sh
python3 ppm_to_bmp.py
```
(You may need to modify the script to point to the correct input file, e.g., `scenario1.ppm`).
