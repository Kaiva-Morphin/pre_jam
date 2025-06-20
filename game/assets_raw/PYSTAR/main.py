from PIL import Image
import random

# Constants
atlas_path = r"R:/prog/pre_jam/game/assets/raw/stars.png"
canvas_size = (512, 512)  # Output texture size
star_size = 5
star_count = 5
instances = 30  # Number of stars to draw

# Load the star atlas and extract sprites
atlas = Image.open(atlas_path).convert("RGBA")
stars = [atlas.crop((i * star_size, 0, (i + 1) * star_size, star_size)) for i in range(star_count)]

# Create the target canvas
canvas = Image.new("RGBA", canvas_size, (0, 0, 0, 0))

# Draw stars
for _ in range(instances):
    star = random.choice(stars)
    x = random.randint(0, canvas_size[0] - 1)
    y = random.randint(0, canvas_size[1] - 1)

    for dx in (-canvas_size[0], 0, canvas_size[0]):
        for dy in (-canvas_size[1], 0, canvas_size[1]):
            canvas.paste(star, ((x + dx) % canvas_size[0], (y + dy) % canvas_size[1]), star)

# Save output
canvas.save("seamless_stars.png")
