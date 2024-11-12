use image::ImageReader;
use std::io::Cursor;

fn main() {
    let img = ImageReader::open("imgs/3.png").unwrap().decode().unwrap();

    let buffer = img
        .into_luma8()
        .into_iter()
        .map(|x| {
            if *x == 251 {
                "0x0000, ".to_owned()
            }
            else if *x == 221 || *x == 81 {
                "0x2945, ".to_owned()
            }
            else if *x == 44 {
                "0xFFFF, ".to_owned()
            }
            else if *x == 154 {
                "0x528A, ".to_owned()
            } else {
                x.to_string() + ", "
            }
        })
        .collect::<String>();

    println!("{:?}", buffer);
}
