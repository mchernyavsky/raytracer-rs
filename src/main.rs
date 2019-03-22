use raytracer::{print_as_ppm, Color, Image, Ray, Vec3, RED};

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().squared_length();
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.squared_length() - radius * radius;
    let discriminant = b.powi(2) - 4.0 * a * c;
    discriminant > 0.0
}

fn color_at(ray: &Ray) -> Color {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let radius = 0.5;

    if hit_sphere(center, radius, ray) {
        return RED;
    }

    let unit_direction = ray.direction().to_unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let color_vec = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);

    let color_scale = 254.99;
    Color::new(
        (color_scale * color_vec.x()) as u8,
        (color_scale * color_vec.y()) as u8,
        (color_scale * color_vec.z()) as u8,
    )
}

fn main() {
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
    print_as_ppm(image);
}
