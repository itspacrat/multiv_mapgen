#![allow(unused_imports)]
use image::{load, GenericImageView};
use serde_json::to_string_pretty;
use modvlo::*;
use {
    hex::*,
    image::{
        imageops::{resize, FilterType},
        io::Reader,
        ImageBuffer, ImageFormat, ImageOutputFormat, Pixel, Rgb, RgbImage, Rgba, RgbaImage,
    },
    //rand::{thread_rng, Rng},
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
pub type Pos2D = [Pos;2];
#[derive(Serialize, Deserialize)]
pub struct MapRgb {
    r: u8,
    g: u8,
    b: u8,
}
pub type Db = HashMap<u8, DbItem>;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("loading db...");
    let db: Db = (from_str(&read_to_string("db.json")?)?).to_owned();
    
    Ok(())
}
