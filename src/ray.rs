use crate::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn point_at_parameter(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
