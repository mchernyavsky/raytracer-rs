use std::fmt::{Display, Error, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Point2D {
    pub x: u32,
    pub y: u32,
}

pub struct Circle {
    pub center: Point2D,
    pub radius: u32,
}

#[derive(Default, Clone, Copy)]
pub struct RGBPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct PPMImage {
    buffer: Box<[RGBPixel]>,
    pub width: u32,
    pub height: u32,
}

impl PPMImage {
    pub fn new(width: u32, height: u32) -> PPMImage {
        PPMImage {
            buffer: vec![RGBPixel::default(); (width * height) as usize].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn draw_circle(&mut self, circle: Circle, fill_with: RGBPixel) {
        let radius_pow2 = circle.radius * circle.radius;
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point2D { x, y };
                if distance_pow2(circle.center, point) < radius_pow2 {
                    self[point] = fill_with
                }
            }
        }
    }

    fn point_to_index(&self, Point2D { x, y }: Point2D) -> usize {
        assert!(x < self.width && y < self.height);
        (y * self.width + x) as usize
    }
}

fn distance_pow2(from: Point2D, to: Point2D) -> u32 {
    ((to.x as i32 - from.x as i32).pow(2) + (to.y as i32 - from.y as i32).pow(2)) as u32
}

impl Index<Point2D> for PPMImage {
    type Output = RGBPixel;

    fn index(&self, point: Point2D) -> &Self::Output {
        &self.buffer[self.point_to_index(point)]
    }
}

impl IndexMut<Point2D> for PPMImage {
    fn index_mut(&mut self, point: Point2D) -> &mut Self::Output {
        &mut self.buffer[self.point_to_index(point)]
    }
}

impl Display for PPMImage {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let point = Point2D { x, y };
                let RGBPixel { red, green, blue } = self[point];
                writeln!(f, "{} {} {}", red, green, blue)?;
            }
        }

        Ok(())
    }
}
