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

fn color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    match world.hit(ray, 0.0, f32::MAX) {
        Some(HitRecord { normal, .. }) => 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0)),
        None => {
            let dir = ray.direction.normalize();
            let t = 0.5 * (dir.y + 1.0);
            vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
        }
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
        c / (ns as f32)
    });
}
