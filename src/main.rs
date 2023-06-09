use ocular::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const LOOKFROM: Point = Point::new(13.0, 2.0, 3.0);
const LOOKAT: Point = Point::new(0.0, 0.0, 0.0);
const CAMERA_UP: Point = Point::new(0.0, 1.0, 0.0);
const FOV: f32 = 20.0;
const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);

const SAMPLES_PER_PIXEL: usize = 64;
const BOUNCES: usize = 16;
const CLIP_START: f32 = 0.01;
const CLIP_END: f32 = f32::INFINITY;
const BLOCK_SIZE: usize = 32;

fn main() {
    let camera = Camera::new(LOOKFROM, LOOKAT, CAMERA_UP, FOV, ASPECT_RATIO);
    let sky = texture::Sky;

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    for i in -11..11 {
        for j in -11..11 {
            let center = Point::new(
                i as f32 + (0.9 * random()),
                0.2,
                j as f32 + (0.9 * random()),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).len() < 0.9 {
                continue;
            }

            let r = random();

            let material: Box<dyn Material> = if r < 0.8 {
                let texture = texture::Solid::new(Color::random_in_unit_sphere());
                let material = material::Diffuse::new(Box::new(texture));
                Box::new(material)
            } else if r < 0.95 {
                let texture = texture::Solid::new(Color::random_in_unit_sphere());
                let material = material::Metal::new(Box::new(texture), random());
                Box::new(material)
            } else {
                let material = material::Glass::new(1.5);
                Box::new(material)
            };

            let sphere = object::Sphere::new(center, 0.2, material);
            objects.push(Box::new(sphere));
        }
    }

    let s1 = {
        let material = material::Glass::new(1.5);
        let sphere = object::Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, Box::new(material));
        Box::new(sphere)
    };

    let s2 = {
        let texture = texture::Solid::new(Color::new(0.4, 0.2, 0.1));
        let material = material::Diffuse::new(Box::new(texture));
        let sphere = object::Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, Box::new(material));
        Box::new(sphere)
    };

    let s3 = {
        let texture = texture::Solid::new(Color::new(0.7, 0.6, 0.5));
        let material = material::Metal::new(Box::new(texture), 0.0);
        let sphere = object::Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, Box::new(material));
        Box::new(sphere)
    };

    let ground = {
        let texture = texture::Solid::new(Color::new(0.5, 0.5, 0.5));
        let material = material::Diffuse::new(Box::new(texture));
        let sphere = object::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material));
        Box::new(sphere)
    };

    objects.push(s1);
    objects.push(s2);
    objects.push(s3);
    objects.push(ground);

    let options = RenderOptions {
        width: WIDTH,
        height: HEIGHT,
        crop_region: ((0, WIDTH), (0, HEIGHT)),
        samples: SAMPLES_PER_PIXEL,
        bounces: BOUNCES,
        clip_start: CLIP_START,
        clip_end: CLIP_END,
        block_size: BLOCK_SIZE,
    };

    let scene = Scene::new(camera, Box::new(sky), objects);
    let image = scene.threaded_render(options);

    let mut bitmap = bmp::Image::new(WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ocular::Pixel { r, g, b } = image.get_pixel(x, y);
            bitmap.set_pixel(x as u32, y as u32, bmp::Pixel { r, g, b });
        }
    }

    bitmap.save("result.bmp").unwrap();
}
