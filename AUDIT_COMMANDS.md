# Audit Commands

Use these commands to generate the images required for the audit.

## 1. Build the Project (Optimized)
```bash
cargo build --release
```

## 2. Generate Audit Scenarios
Run each command to generate the corresponding image.

### Scenario 1: Sphere
```bash
cargo run --release --bin scenario1 > scenario1.ppm
```

### Scenario 2: Plane + Cube (Low Brightness)
```bash
cargo run --release --bin scenario2 > scenario2.ppm
```

### Scenario 3: All Objects (Sphere, Cube, Cylinder, Plane)
```bash
cargo run --release --bin scenario3 > scenario3.ppm
```

### Scenario 4: All Objects (Different Perspective)
```bash
cargo run --release --bin scenario4 > scenario4.ppm
```

## 3. Convert Images for Windows (BMP)
Since `.ppm` files are not natively supported on Windows, run this Python script to convert all generated images to `.bmp`:

```bash
python3 ppm_to_bmp.py
```
This will create `scenario1.bmp`, `scenario2.bmp`, etc., which you can open with double-click.
