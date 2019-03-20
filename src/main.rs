use raytracer::{Circle, PPMImage, Point2D, RGBPixel};

fn main() {
    let mut image = PPMImage::new(200, 100);
    image.draw_circle(
        Circle {
            center: Point2D { x: 100, y: 50 },
            radius: 25,
        },
        RGBPixel {
            red: 200,
            green: 200,
            blue: 0,
        },
    );
    print!("{}", image)
}
