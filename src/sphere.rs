use crate::vectors::Vector3;
use crate::rays::Ray;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin().subtract(&self.center);
        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let root = (-half_b - sqrtd) / a;
            if root < 0.0 {
                None
            } else {
                Some(root)
            }
        }
    }
}
