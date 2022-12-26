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

        // for i in x - 1..x + 1 as i64 {
        //     for j in y - 1..y + 1 as i64 {
        //         if i < img.get_width() && j < img.get_height() {
        //             red = img.get_pixel(i, j).r + red;
        //             green = img.get_pixel(i, j).g + green;
        //             blue = img.get_pixel(i, j).b + green;
        //             n = n + 1.0;
        //         } else {
        //             continue;
        //         }
        //     }
        // }
        let x2: i64 = x as i64;
        let y2: i64 = y as i64;
        let mut i: i64 = (x2 - 1) as i64;
        loop {
            if i <= (x + 1) as i64 {
                let mut j: i64 = (y2 - 1) as i64;

                loop {
                    if j <= (y + 1) as i64 {
                        if (i as u32) < img.get_width() && (j as u32) < img.get_height() {
                            red = img.get_pixel(i as u32, j as u32).r + red;
                            green = img.get_pixel(i as u32, j as u32).g + green;
                            blue = img.get_pixel(i as u32, j as u32).b + green;
                            n = n + 1.0;
                        } else {
                            continue;
                        }
                    } else {
                        break;
                    }
                    j += 1;
                }
            } else {
                break;
            }
            i += 1;
        }
        img.set_pixel(
            x,
            y,
            px!((red / n as u8), (green / n as u8), (blue / n as u8)),
        );
    }
    let _ = img.save("imgblured.bmp");
}
