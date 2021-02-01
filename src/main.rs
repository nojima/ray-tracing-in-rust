mod hit;
mod ray;
mod vec3;

use hit::*;
use ray::*;
use vec3::*;

fn write_header(nx: i32, ny: i32) {
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
}

fn write_body(nx: i32, ny: i32, plot: impl Fn(i32, i32) -> Vec3) {
    for y in (0..ny).rev() {
        for x in 0..nx {
            let color = plot(x, y);
            let ir = (255.99 * color.x) as i32;
            let ig = (255.99 * color.y) as i32;
            let ib = (255.99 * color.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    match world.hit(ray, 0.0, f32::MAX) {
        Some(HitRecord { normal, .. }) => 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0)),
        None => {
            let dir = ray.direction.normalize();
            let t = 0.5 * (dir.y + 1.0);
            lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
        }
    }
}

fn main() {
    let (nx, ny) = (200, 100);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
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
    write_header(nx, ny);
    write_body(nx, ny, |x, y| {
        let u = (x as f32) / (nx as f32);
        let v = (y as f32) / (ny as f32);
        let ray = Ray {
            origin,
            direction: lower_left_corner + u * horizontal + v * vertical,
        };
        color(&ray, &world)
    });
}
