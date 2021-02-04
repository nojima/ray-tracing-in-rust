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

pub struct Dielectric {
    pub refractive_index: f32,
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, p: Vec3, normal: Vec3) -> Option<ScatterResult> {
        let (outward_normal, ni_over_nt, cosine) = if ray_in.direction.dot(normal) > 0.0 {
            (
                -normal,
                self.refractive_index,
                self.refractive_index * ray_in.direction.dot(normal) / ray_in.direction.length(),
            )
        } else {
            (
                normal,
                1.0 / self.refractive_index,
                -ray_in.direction.dot(normal) / ray_in.direction.length(),
            )
        };
        if let Some(refracted) = ray_in.direction.refract(outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refractive_index);
            if random::<f32>() >= reflect_prob {
                return Some(ScatterResult {
                    scattered: Ray {
                        origin: p,
                        direction: refracted,
                    },
                    attenuation: Vec3::new(1.0, 1.0, 1.0),
                });
            }
        }
        Some(ScatterResult {
            scattered: Ray {
                origin: p,
                direction: ray_in.direction.reflect(normal),
            },
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        })
    }
}
