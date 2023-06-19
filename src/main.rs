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

    println!("\n\n---\n* loading db...");
    let db = from_str::<Db>(&read_to_string("db.json")?)?;

    println!("* loading image data...");
    let img = open("process/map/test1d/input.png")?.to_rgb8();
    println!("{:?}", img);

    println!("* creating map boxes...");
    let mut tiles: Vec<u8> = Vec::new();
    
    println!("* pushing data [{}]...",img.width()*img.height());
    for (p) in img.pixels() {
        for (dbindex, dbitem) in &db {
            if dbitem.rgb == [p[0],p[1],p[2]] {
                println!("({}): {:?} == {:?}",dbindex,dbitem.rgb,[p[0],p[1],p[2]]);
                tiles.push(*dbindex);
            }
        }
    }

    let mut out_img = RgbImage::new(img.width(),img.height());
    
    for (i,t) in tiles.iter().enumerate() {
        out_img.put_pixel(
            //x
            ((i as u32 %img.width())) as u32,
            //y
            (i as f32/img.width() as f32).floor() as u32,
            //pix
            *Pixel::from_slice(&(db.get(t).unwrap().rgb))
        );
    }
    
    println!("\n* done.");
    
    for i in 0..tiles.len() {
        if i % img.width() as usize == 0 {
            print!("\n{}",tiles[i])
        } else {print!("{}",tiles[i])}
    }

    // for debugs
    //out_img.save_with_format("process/map/test1d/out.png", ImageFormat::Png)?;
    println!();
   
    Ok(())
}
