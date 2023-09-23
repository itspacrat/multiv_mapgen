#![allow(unused_imports)]
use image::{load, open, GenericImageView};
use lib_multiv::*;

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
    rgb: MvRGB,
    attributes: Vec<String>,
}

pub type Db = HashMap<u8, DBItem>;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let input: Vec<String> = from_str(&read_to_string("process.json")?)?;

    for r in input {
        println!("loading {}",&r);
        let save_map = gen_map(format!("process/map/{}/input.png",r.clone()))?;
        let out_string = to_string_pretty(&save_map)?;
        let _ = write(format!("process/map/{}/data.json",&r),out_string);
        println!("\n\n{} done.\n", &r);
    }

    Ok(())
}
