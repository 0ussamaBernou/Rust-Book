#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};
fn main() {
    let mut img: Image = match bmp::open("./images/yard.bmp") {
        Ok(img) => img,
        Err(_) => return,
    };
    for (x, y) in img.coordinates() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut n = 0.0;

        let x2: i64 = x as i64;
        let y2: i64 = y as i64;

        for i in x2 - 1..x2 + 1 {
            for j in y2 - 1..y2 + 1 {
                if (i as u32) < img.get_width() && (j as u32) < img.get_height() {
                    red = img.get_pixel(i as u32, j as u32).r + red;
                    green = img.get_pixel(i as u32, j as u32).g + green;
                    blue = img.get_pixel(i as u32, j as u32).b + green;
                    n = n + 1.0;
                } else {
                    continue;
                }
            }
        }
        img.set_pixel(
            x,
            y,
            px!((red / n as u8), (green / n as u8), (blue / n as u8)),
        );
    }
    let _ = img.save("imgblured.bmp");
}
