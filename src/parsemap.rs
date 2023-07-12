use crate::*;
pub type Img = RgbImage;
pub type Map = Vec<u8>;

pub fn img_to_map(data: Img) -> Map {
    unimplemented!()
}
pub fn map_to_img(data: Map) -> Img {
    unimplemented!()
}
pub fn gen_map(input: String) -> Result<ShvftMap, Box<dyn Error>> {
    println!("\n\n---\n* loading db...");
    let db = from_str::<Db>(&read_to_string("db.json")?)?;

    println!("* loading image data...");
    let img = open(input)?.to_rgb8();
    println!("{:?}", img);

    println!("* creating map boxes...");
    let mut tiles: Vec<u8> = Vec::new();

    println!("* pushing data [{}]...", img.width() * img.height());
    for (p) in img.pixels() {
        for (dbindex, dbitem) in &db {
            if dbitem.rgb == [p[0], p[1], p[2]] {
                println!(
                    "({}): {:?} == {:?}",
                    dbindex,
                    dbitem.rgb,
                    [p[0], p[1], p[2]]
                );
                tiles.push(*dbindex);
            }
        }
    }

    let mut out_img = RgbImage::new(img.width(), img.height());

    for (i, t) in tiles.iter().enumerate() {
        out_img.put_pixel(
            //x
            (i as u32 % img.width()) as u32,
            //y
            (i as f32 / img.width() as f32).floor() as u32,
            //pix
            *Pixel::from_slice(&(db.get(t).unwrap().rgb)),
        );
    }

    println!("\n* done.");

    for i in 0..tiles.len() {
        if i % img.width() as usize == 0 {
            print!("\n{}", tiles[i])
        } else {
            print!("{}", tiles[i])
        }
    }

    // for debugs
    //out_img.save_with_format("process/map/test1d/out.png", ImageFormat::Png)?;

    let save_map = ShvftMap {
        width: img.width() as usize,
        tiles,
        doors: vec![],
        notes: vec![],
        containers: vec![],
    };

    Ok(save_map)
}
