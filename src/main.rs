#![allow(unused_imports)]
use image::{load, open, GenericImageView};
use modvlo::*;
use serde_json::to_string_pretty;
use {
    hex::*,
    image::{
        imageops::{resize, FilterType},
        io::Reader,
        ImageBuffer, ImageFormat, ImageOutputFormat, Pixel, Rgb, RgbImage, Rgba, RgbaImage,
    },
    serde::{Deserialize, Serialize},
    serde_json::{from_str, to_string, to_value, Value},
    std::{
        array,
        collections::HashMap,
        env,
        error::Error,
        fs::{read_to_string, write, File, OpenOptions},
        io::{Read, Write},
        path::Path,
    },
};
pub type Pos = usize;

#[derive(Serialize, Deserialize)]
pub struct DBItem {
    description: String,
    rgb: ShvftRgb,
    attributes: Vec<String>,
}
pub type Db = HashMap<u8, DBItem>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("loading db...");
    let db: Db = (from_str::<Db>(&read_to_string("db.json")?)?).to_owned();

    let img = open("process/map/test1d/input.png")?;
    let mut imgout = RgbImage::new(img.width(), img.height());
    let mut tiles: Vec<Rgba<u8>> = Vec::new();
    for (y) in 0_u32..(imgout.height() as usize) {
        for x in 0_u32..(imgout.width() as usize) {
            // add pixel to vec
            let [r, g, b] = [
                img.get_pixel(x, y)[0],
                img.get_pixel(x, y)[1],
                img.get_pixel(x, y)[2],
            ];
            let push: u8;
            for (id, dbitem) in db {
                match [dbitem.rgb[0], dbitem.rgb[1], dbitem.rgb[2]] {
                    [r, g, b] => img.push(id),
                    _ => {}
                }
            }
        }
    }

    //println!("{:?}",tiles);
    Ok(())
}
