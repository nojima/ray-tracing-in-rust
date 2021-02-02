use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, p: Vec3, normal: Vec3) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let x = random::<f32>() * 2.0 - 1.0;
        let y = random::<f32>() * 2.0 - 1.0;
        let z = random::<f32>() * 2.0 - 1.0;
        if x * x + y * y + z * z < 1.0 {
            return Vec3::new(x, y, z);
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, p: Vec3, normal: Vec3) -> Option<ScatterResult> {
        let target = p + normal + random_in_unit_sphere();
        Some(ScatterResult {
            scattered: Ray {
                origin: p,
                direction: target - p,
            },
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, p: Vec3, normal: Vec3) -> Option<ScatterResult> {
        let reflected = ray_in.direction.normalize().reflect(normal);
        let scattered = reflected + self.fuzz * random_in_unit_sphere();
        if scattered.dot(normal) > 0.0 {
            Some(ScatterResult {
                scattered: Ray {
                    origin: p,
                    direction: scattered,
                },
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
