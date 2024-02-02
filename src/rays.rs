use crate::vectors::Vector3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}


impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin.add(&self.direction.multiply(t))
    }
}