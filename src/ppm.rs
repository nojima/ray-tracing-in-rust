use crate::vec3::Vec3;

pub fn write_header(nx: i32, ny: i32) {
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
}

pub fn write_body(nx: i32, ny: i32, plot: impl Fn(i32, i32) -> Vec3) {
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
