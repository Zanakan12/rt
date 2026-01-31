import sys

def ppm_to_bmp(ppm_path, bmp_path):
    with open(ppm_path, 'r') as f:
        lines = f.readlines()

    # Filter out comments and clean data
    data = []
    for line in lines:
        line = line.split('#')[0].strip()
        if line:
            data.extend(line.split())

    if data[0] != 'P3':
        print("Not a P3 PPM file")
        return

    width = int(data[1])
    height = int(data[2])
    max_val = int(data[3])

    pixel_data = [int(x) for x in data[4:]]
    
    # BMP Header
    file_size = 14 + 40 + (width * height * 3) + (height * (width % 4)) 
    # Padding bytes to ensure rows are multiples of 4 bytes
    padding = (4 - (width * 3) % 4) % 4
    file_size_with_padding = 14 + 40 + (height * (width * 3 + padding))
    
    bmp_header = bytearray([
        0x42, 0x4D,             # Signature 'BM'
        file_size_with_padding & 0xFF, (file_size_with_padding >> 8) & 0xFF, (file_size_with_padding >> 16) & 0xFF, (file_size_with_padding >> 24) & 0xFF,
        0, 0, 0, 0,             # Reserved
        54, 0, 0, 0             # Offset to pixel data (14 + 40)
    ])

    dib_header = bytearray([
        40, 0, 0, 0,            # Header size
        width & 0xFF, (width >> 8) & 0xFF, (width >> 16) & 0xFF, (width >> 24) & 0xFF,
        height & 0xFF, (height >> 8) & 0xFF, (height >> 16) & 0xFF, (height >> 24) & 0xFF,
        1, 0,                   # Planes
        24, 0,                  # Bits per pixel (RGB)
        0, 0, 0, 0,             # Compression (None)
        0, 0, 0, 0,             # Image size (can be 0 for no compression)
        0, 0, 0, 0,             # X pixels per meter
        0, 0, 0, 0,             # Y pixels per meter
        0, 0, 0, 0,             # Colors in color table
        0, 0, 0, 0,             # Important color count
    ])

    with open(bmp_path, 'wb') as f:
        f.write(bmp_header)
        f.write(dib_header)

        # BMP stores pixels bottom-to-top, but our PPM is top-to-bottom
        # So we process rows in reverse order
        
        # Group pixels into (r, g, b) tuples
        pixels = []
        for i in range(0, len(pixel_data), 3):
            if i+2 < len(pixel_data):
                pixels.append((pixel_data[i], pixel_data[i+1], pixel_data[i+2]))
        
        # Check if we have enough pixels
        if len(pixels) != width * height:
            print(f"Warning: Expected {width*height} pixels, got {len(pixels)}")

        pad_bytes = bytearray([0] * padding)

        for y in range(height - 1, -1, -1):
            row_start = y * width
            row_end = row_start + width
            row_pixels = pixels[row_start:row_end]
            
            for r, g, b in row_pixels:
                # Scale to 0-255 if max_val isn't 255 (though our raytracer uses 255)
                if max_val != 255:
                    r = int(r * 255 / max_val)
                    g = int(g * 255 / max_val)
                    b = int(b * 255 / max_val)
                # BMP uses BGR format
                f.write(bytearray([b, g, r]))
            
            f.write(pad_bytes)

    print(f"Converted {ppm_path} to {bmp_path}")

if __name__ == "__main__":
    files = ["output.ppm", "scenario1.ppm", "scenario2.ppm", "scenario3.ppm", "scenario4.ppm"]
    for f in files:
        try:
             bmp_name = f.replace(".ppm", ".bmp")
             ppm_to_bmp(f, bmp_name)
        except Exception as e:
            print(f"Could not convert {f}: {e}")
