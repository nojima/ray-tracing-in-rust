mod camera;
mod hit;
mod ppm;
mod ray;
mod vec3;

use camera::Camera;
use hit::*;
use rand::prelude::*;
use ray::Ray;
use vec3::Vec3;

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

fn color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    match world.hit(ray, 0.001, f32::MAX) {
        Some(HitRecord { p, normal, .. }) => {
            let target = p + normal + random_in_unit_sphere();
            let new_ray = Ray {
                origin: p,
                direction: target - p,
            };
            0.5 * color(&new_ray, world)
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
    let camera = Camera::new();
    let world = HitList::new(vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ]);
    ppm::write_header(nx, ny);
    ppm::write_body(nx, ny, |x, y| {
        let mut c = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f32 + random::<f32>()) / (nx as f32);
            let v = (y as f32 + random::<f32>()) / (ny as f32);
            let ray = camera.get_ray(u, v);
            c = c + color(&ray, &world);
        }
        let linear_color = c / (ns as f32);
        gamma_correction(linear_color)
    });
}
