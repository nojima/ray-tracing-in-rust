fn write_header(nx: i32, ny: i32) {
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
}

fn write_body(nx: i32, ny: i32) {
    for y in (0..ny).rev() {
        for x in 0..nx {
            let r = (x as f32) / (nx as f32);
            let g = (y as f32) / (ny as f32);
            let b = 0.2;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    let (nx, ny) = (200, 100);
    write_header(nx, ny);
    write_body(nx, ny);
}
