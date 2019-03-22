use crate::Color;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Deref, Index, IndexMut};

type Point = (u32, u32);

pub struct Image {
    buffer: Box<[Color]>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_background(width, height, Color::default())
    }

    pub fn with_background(width: u32, height: u32, background: Color) -> Self {
        Self {
            buffer: vec![background; (width * height) as usize].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn point_to_index(&self, (x, y): Point) -> usize {
        assert!(x < self.width && y < self.height);
        (y * self.width + x) as usize
    }
}

impl Index<Point> for Image {
    type Output = Color;

    fn index(&self, point: Point) -> &Self::Output {
        &self.buffer[self.point_to_index(point)]
    }
}

impl IndexMut<Point> for Image {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.buffer[self.point_to_index(point)]
    }
}

struct PPMImage(Image);

impl Deref for PPMImage {
    type Target = Image;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for PPMImage {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let color = self[(x, y)];
                writeln!(f, "{} {} {}", color.red(), color.green(), color.blue())?;
            }
        }

        Ok(())
    }
}

pub fn print_as_ppm(image: Image) {
    print!("{}", PPMImage(image))
}
