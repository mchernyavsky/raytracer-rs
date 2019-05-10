use crate::Color;
use std::io::{Error, Write};
use std::ops::{Index, IndexMut};

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
        Self::with_data(
            width,
            height,
            vec![background; (width * height) as usize].into_boxed_slice(),
        )
    }

    pub fn with_data(width: u32, height: u32, buffer: Box<[Color]>) -> Self {
        assert_eq!(width * height, buffer.len() as u32);
        Self {
            buffer,
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

pub fn write_ppm<W: Write>(image: Image, output: &mut W) -> Result<(), Error> {
    writeln!(output, "P3")?;
    writeln!(output, "{} {}", image.width, image.height)?;
    writeln!(output, "255")?;

    for y in (0..image.height).rev() {
        for x in 0..image.width {
            let color = image[(x, y)];
            writeln!(output, "{} {} {}", color.red(), color.green(), color.blue())?;
        }
    }

    Ok(())
}
