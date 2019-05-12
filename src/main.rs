#![feature(try_blocks)]

use rayon::prelude::*;
use std::io::Write;
use std::{env, io};

use rand::Rng;
use raytracer::{
    write_ppm, Camera, Color, Hit, HitList, Image, Lambertian, Metal, Ray, Sphere, Vec3, RED,
};
use std::fs::File;
use std::sync::Arc;

enum Error {
    ParseError(String),
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

fn handle_error(err: &Error) -> i32 {
    match err {
        Error::ParseError(parse_err_msg) => eprintln!("{}", parse_err_msg),
        Error::IOError(io_err) => eprintln!("{}", io_err),
    };
    1
}

fn get_output_file_name() -> Result<Option<String>, Error> {
    let args: Vec<_> = env::args().collect();
    let filename = match args.len() {
        1 => None,
        2 => Some(args[1].to_string()),
        _ => Err(Error::ParseError("Too many arguments".to_string()))?,
    };
    Ok(filename)
}

fn draw_sphere<W: Write>(output: &mut W) -> Result<(), Error> {
    let camera = make_camera();
    let world = make_world();
    let mut image = Image::with_background(IMAGE_WIDTH, IMAGE_HEIGHT, RED);
    image
        .pixels()
        .enumerate()
        .for_each(|(idx, pixel)| *pixel = calc_pixel(idx, &camera, &world));
    write_ppm(image, output)?;
    return Ok(());

    const IMAGE_WIDTH: u32 = 200;
    const IMAGE_HEIGHT: u32 = 100;

    fn make_camera() -> Camera {
        Camera::new(
            Vec3::new(-2.0, -1.0, -1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        )
    }

    fn make_world() -> HitList {
        let mut world = HitList::new();
        world.push(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        ));
        world.push(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        ));
        world.push(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        ));
        world.push(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0)),
        ));
        world
    }

    fn calc_pixel(idx: usize, camera: &Camera, world: &HitList) -> Color {
        const N_SAMPLES: u32 = 100;
        let mut rng = rand::thread_rng();
        let x = idx as u32 % IMAGE_WIDTH;
        let y = idx as u32 / IMAGE_WIDTH;
        let mut acc = Vec3::default();
        for _ in 0..N_SAMPLES {
            let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH);
            let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT);
            let ray = camera.get_ray(u, v);
            acc += color_vec_at(&ray, world, 0);
        }
        return vec_to_color(acc / f64::from(N_SAMPLES));

        fn color_vec_at<T: Hit>(ray: &Ray, world: &T, depth: u32) -> Vec3 {
            const MAX_DEPTH: u32 = 50;
            if let Some(hit) = world.hit(ray, 0.001, std::f64::MAX) {
                if depth >= MAX_DEPTH {
                    return Vec3::default();
                }

                if let Some(scattered) = hit.material().scatter(ray, &hit) {
                    scattered.attenuation() * color_vec_at(&scattered.ray(), world, depth + 1)
                } else {
                    Vec3::default()
                }
            } else {
                let unit_direction = ray.direction().normalize();
                let t = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
            }
        }

        fn vec_to_color(vec: Vec3) -> Color {
            const COLOR_SCALE: f64 = 254.99;
            let vec = COLOR_SCALE * Vec3::new(vec.x().sqrt(), vec.y().sqrt(), vec.z().sqrt());
            Color::new(vec.x() as u8, vec.y() as u8, vec.z() as u8)
        }
    }
}

fn main() {
    let exit_code = {
        let res = try {
            if let Some(filename) = get_output_file_name()? {
                draw_sphere(&mut File::create(filename)?)
            } else {
                draw_sphere(&mut io::stdout().lock())
            }?
        };

        match res {
            Ok(()) => 0,
            Err(err) => {
                handle_error(&err);
                1
            }
        }
    };

    std::process::exit(exit_code)
}
