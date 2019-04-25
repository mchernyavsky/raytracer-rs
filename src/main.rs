#![feature(try_blocks)]

extern crate rand;

use std::fs::File;
use std::io::Write;
use std::{env, io};

use rand::distributions::{Distribution, UnitSphereSurface};
use rand::Rng;
use raytracer::{write_ppm, Camera, Color, Hit, HitList, Image, Ray, Sphere, Vec3, RED};

fn color_vec_at<T: Hit>(ray: &Ray, world: &T) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.001, std::f64::MAX) {
        let target = record.point() + record.normal() + random_in_unit_sphere();
        0.5 * color_vec_at(&Ray::new(record.point(), target - record.point()), world)
    } else {
        let unit_direction = ray.direction().to_unit_vec();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let [x, y, z] = sphere.sample(&mut rand::thread_rng());
    Vec3::new(x, y, z)
}

fn vec_to_color(vec: Vec3) -> Color {
    const COLOR_SCALE: f64 = 254.99;
    let vec = COLOR_SCALE * Vec3::new(vec.x().sqrt(), vec.y().sqrt(), vec.z().sqrt());
    Color::new(vec.x() as u8, vec.y() as u8, vec.z() as u8)
}

enum Error {
    ParseError(String),
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
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
    let mut rng = rand::thread_rng();
    let mut image = Image::with_background(200, 100, RED);
    let n_samples = 100;

    let camera = Camera::new(
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let mut world = HitList::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for y in (0..image.height()).rev() {
        for x in 0..image.width() {
            let mut acc = Vec3::default();
            for _ in 0..n_samples {
                let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(image.width());
                let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(image.height());
                let ray = camera.get_ray(u, v);
                acc += color_vec_at(&ray, &world);
            }
            image[(x, y)] = vec_to_color(acc / f64::from(n_samples));
        }
    }

    write_ppm(image, output)?;
    Ok(())
}

fn handle_error(err: &Error) -> i32 {
    match err {
        Error::ParseError(parse_err_msg) => eprintln!("{}", parse_err_msg),
        Error::IOError(io_err) => eprintln!("{}", io_err),
    };
    1
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
