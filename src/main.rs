#![feature(try_blocks)]

use std::fs::File;
use std::io::Write;
use std::{env, io};

use raytracer::{write_ppm, Color, Hitable, Image, Ray, Sphere, Vec3, RED};

fn color_at(ray: &Ray) -> Color {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let color_vec = if let Some(record) = sphere.hit(ray, 0.0, std::f32::MAX) {
        0.5 * (record.normal() + Vec3::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = ray.direction().to_unit_vec();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    };
    let color_scale = 254.99;
    vec_to_color(color_scale * color_vec)
}

fn vec_to_color(vec: Vec3) -> Color {
    Color::new(vec.x() as u8, vec.y() as u8, vec.z() as u8)
}

enum Error {
    ParseError(ParseError),
    IOError(io::Error),
}

struct ParseError(String);

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseError(err)
    }
}

fn get_output_file_name() -> Result<Option<String>, Error> {
    let args: Vec<_> = env::args().collect();
    let filename = match args.len() {
        1 => None,
        2 => Some(args[1].to_string()),
        _ => Err(ParseError("Too many arguments".to_string()))?,
    };
    Ok(filename)
}

fn draw_sphere<W: Write>(output: &mut W) -> Result<(), Error> {
    let width = 200;
    let height = 100;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let mut image = Image::with_background(width, height, RED);
    for y in (0..image.height()).rev() {
        for x in 0..image.width() {
            let u = x as f32 / image.width() as f32;
            let v = y as f32 / image.height() as f32;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            image[(x, y)] = color_at(&ray);
        }
    }

    write_ppm(image, output)?;
    Ok(())
}

fn handle_error(err: &Error) -> i32 {
    match err {
        Error::ParseError(ParseError(parse_err_msg)) => eprintln!("{}", parse_err_msg),
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
            Ok(_) => 0,
            Err(err) => {
                handle_error(&err);
                1
            }
        }
    };

    std::process::exit(exit_code)
}
