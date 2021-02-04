use std::f32::consts::PI;

use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        let lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        let horizontal = Vec3::new(2.0 * half_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0 * half_height, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
