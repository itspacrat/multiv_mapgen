#![allow(unused_imports)]
use image::{load, open, GenericImageView};
use modvlo::*;

use {
    hex::*,
    image::{
        imageops::{resize, FilterType},
        io::Reader,
        ImageBuffer, ImageFormat, ImageOutputFormat, Pixel, Rgb, RgbImage, Rgba, RgbaImage,
    },
    serde::{Deserialize, Serialize},
    serde_json::{from_str, to_string, to_string_pretty, to_value, Value},
    std::{
        array,
        collections::HashMap,
        env,
        error::Error,
        fs::{read_to_string, write, File, OpenOptions},
        io::{Read, Write},
        path::Path,
    },
    parsemap::*
};
pub mod parsemap;
pub type Pos = usize;

#[derive(Serialize, Deserialize)]
pub struct DBItem {
    description: String,
    rgb: ShvftRgb,
    attributes: Vec<String>,
}

pub type Db = HashMap<u8, DBItem>;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let input = String::from("process/map/test1d/input.png");


    let save_map = gen_map(input.clone());
    let out_string = to_string_pretty(&save_map?)?;

    let _ = write("process/map/test1d/map.json",out_string);
    println!();
   
    Ok(())
}
