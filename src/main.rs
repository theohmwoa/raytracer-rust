extern crate image;

mod vectors;
mod rays;
mod sphere;
mod Camera;

use image::{ImageBuffer, RgbImage, Rgb};
use vectors::Vector3;
use rays::Ray;
use sphere::Sphere;
use Camera::Camera as OtherCamera;

fn main() {
    let image_width = 400;
    let image_height = 200;

    let sphere = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let mut max_distance = 0.0;

    let camera = OtherCamera::new();

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = (image_height - j) as f64 / image_height as f64;
            let ray = camera.get_ray(u, v);
            if let Some(distance) = sphere.hit(&ray) {
                if distance > max_distance {
                    max_distance = distance;
                }
            }
        }
    }

    if max_distance == 0.0 {
        max_distance = 1.0;
    }


    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = (image_height - j) as f64 / image_height as f64;
            let ray = camera.get_ray(u, v);
            if let Some(distance) = sphere.hit(&ray) {
                let intensity = 1.0 - (distance / max_distance).min(1.0);
                let color = [255, (255.0 * intensity) as u8, (255.0 * intensity) as u8];
                img.put_pixel(i, j, Rgb(color));
            } else {
                img.put_pixel(i, j, Rgb([255, 255, 255]));
            }
        }
    }

    img.save("output.png").unwrap();
}
