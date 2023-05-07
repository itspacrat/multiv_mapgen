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
    println!("loading db");
    let db: Db = from_str(&read_to_string("db.json")?)?;
    let maps: Vec<String> = from_str(&read_to_string("process.json")?)?;
    for map in maps {
        let mut containers_template: Vec<ShvftContainer> = vec![];
        let mut notes_template: Vec<ShvftNote>= vec![];
        let mut doors_template: Vec<ShvftDoor>= vec![];
        let image = Reader::open(format!("process/map/{}/input.png", &map))
            .unwrap()
            .decode()
            .unwrap();
        println!("loading image");
        let (x_max, y_max) = (image.width() as usize, image.height() as usize);
        println!("resizing data vectors");
        let mut map_data: Vec<Vec<u8>> = Vec::new();
        let mut y_vec: Vec<u8> = Vec::new();
        y_vec.resize(y_max, 0);
        map_data.resize(x_max, y_vec.clone());
        println!("parsing tiles from image data");
        println!("[{}] pushing keys...",map);
    for (x, y, rgb) in image.pixels() {
        let match_arr = rgb.0;
        for (&key, item) in &db {
            if rgb.0[0] == item.rgb.r && rgb.0[1] == item.rgb.g && rgb.0[2] == item.rgb.b {
                match key {
                    4 | 5 => {
                        doors_template.push(ShvftDoor {
                            here: [x as usize,y as usize],
                            there: [0,0],
                            exit_direction: '.',
                            exit_map: String::from("Map0")
                        });
                    }
                    6 => { 
                        containers_template.push(ShvftContainer {
                            pos: [x as usize,y as usize],
                            inventory: vec![0,0]
                        });
                    }
                    8 => {
                        notes_template.push(ShvftNote {
                            pos: [x as usize,y as usize],
                            content: String::from("```\na blank piece of notebook paper.\n```")
                        });
                    }
                    _ => {/*edge case, nonmatching key */}
                }
                /* add db item to map */
                map_data[x as usize][y as usize] = key;
                /*
                println!("pushed key @ {},{}: {}", &x, &y, key);
                println!("key2 {}",key);
                */
            } else {
            }
        }
    }
    let final_map = map_data.iter().map(|line|encode(line)).collect::<Vec<_>>();

    let _ = write(format!(
        "process/map/{}/tiles.json",&map),
        to_string_pretty(&final_map).unwrap().as_bytes()
    ).unwrap();
    let _ = write(format!(
        "process/map/{}/containers.json",&map),
        to_string(&containers_template).unwrap().as_bytes()
    ).unwrap();
    let _ = write(format!(
        "process/map/{}/notes.json",&map),
        to_string(&notes_template).unwrap().as_bytes()
    ).unwrap();
    let _ = write(format!(
        "process/map/{}/doors.json",&map),
        to_string(&doors_template).unwrap().as_bytes()
    ).unwrap();
    }
    println!("all done.");
    Ok(())
}
