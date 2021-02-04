mod camera;
mod hit;
mod material;
mod ppm;
mod ray;
mod vec3;

use camera::Camera;
use hit::*;
use material::*;
use rand::prelude::*;
use ray::Ray;
use vec3::Vec3;

fn color(ray: Ray, world: &dyn Hit, depth: i32) -> Vec3 {
    match world.hit(&ray, 0.001, f32::MAX) {
        Some(HitRecord {
            p,
            normal,
            material,
            ..
        }) => {
            if depth >= 50 {
                return Vec3::new(0.0, 0.0, 0.0);
            }
            match material.scatter(&ray, p, normal) {
                Some(ScatterResult {
                    scattered,
                    attenuation,
                }) => attenuation * color(scattered, world, depth + 1),
                None => Vec3::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let dir = ray.direction.normalize();
            let t = 0.5 * (dir.y + 1.0);
            vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
        }
    }
}

fn gamma_correction(c: Vec3) -> Vec3 {
    Vec3 {
        x: c.x.sqrt(),
        y: c.y.sqrt(),
        z: c.z.sqrt(),
    }
}

fn main() {
    let (nx, ny) = (200, 100);
    let ns = 100;
    let camera = Camera::new(45.0, nx as f32 / ny as f32);
    let world = HitList::new(vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Lambertian {
                albedo: Vec3::new(0.8, 0.3, 0.3),
            }),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Box::new(Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            }),
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.0,
            }),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Dielectric {
                refractive_index: 1.5,
            }),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Box::new(Dielectric {
                refractive_index: 1.5,
            }),
        }),
    ]);
    ppm::write_header(nx, ny);
    ppm::write_body(nx, ny, |x, y| {
        let mut c = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f32 + random::<f32>()) / (nx as f32);
            let v = (y as f32 + random::<f32>()) / (ny as f32);
            let ray = camera.get_ray(u, v);
            c = c + color(ray, &world, 0);
        }
        let linear_color = c / (ns as f32);
        gamma_correction(linear_color)
    });
}
