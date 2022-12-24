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

        for i in x - 1..x + 1 {
            for j in y - 1..y + 1 {
                if i < img.get_width() && j < img.get_height() {
                    red = img.get_pixel(i, j).r + red;
                    green = img.get_pixel(i, j).g + green;
                    blue = img.get_pixel(i, j).b + green;
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
