use crate::vectors::Vector3;
use crate::rays::Ray;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
        let horizontal = Vector3::new(4.0, 0.0, 0.0);
        let vertical = Vector3::new(0.0, 2.0, 0.0);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner.add(&self.horizontal.multiply(u)).add(&self.vertical.multiply(v)).subtract(&self.origin);
        Ray::new(self.origin.clone(), direction)
    }
}