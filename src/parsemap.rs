use crate::*;
pub type Img = RgbImage;
pub type Map = Vec<u8>;

pub fn img_to_map(data: Img) -> Map {
    unimplemented!()
    // TODO implement
}
pub fn map_to_img(data: Map) -> Img {
    unimplemented!()
    // TODO implement
}
pub fn gen_map(input: String) -> Result<MvRoom, Box<dyn Error>> {
    println!("\n\n---\n* loading db...");
    let db = from_str::<Db>(&read_to_string("db.json")?)?;

    println!("* loading image data...");
    let img = open(input.clone())?.to_rgb8();
    //println!("{:?}", img);

    println!("* creating map boxes...");
    let mut tiles: Vec<u8> = Vec::new();

    let mut doors: HashMap<String,MvDoor> = HashMap::new();
    //let mut containers: Vec<MvBox> = Vec::new();
    let mut notes: HashMap<Pos,String> = HashMap::new();

    println!("* pushing data [{}]...", img.width() * img.height());
    for p in img.pixels() {
        for (dbindex, dbitem) in &db {
            if dbitem.rgb == [p[0], p[1], p[2]] {
                /*println!(
                    "({}): {:?} == {:?}",
                    dbindex,
                    dbitem.rgb,
                    [p[0], p[1], p[2]]
                );*/
                tiles.push(*dbindex);
                
            }
        }
    }

    for (index, tile) in tiles.iter().enumerate() {

        match *tile {
            4|5 => {
                doors.insert(format!("{index}"),MvDoor{
                    here: index,
                    there: 0,
                    exit_map: format!("exit"),
                    exit_direction: '.'
                });
            }
            /*6 => {
                containers.push(ShvftContainer {
                    pos: index,
                    inventory: vec![0,0,0]
                })
            }*/

            8 => {
                notes.insert(index,format!("```\n\n📝 a blank piece of notebook paper.\n\n```"));
            }

            _ => {}
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
    // todo: use substring or somethign 3head
    let input_dir = input.clone()[12..]
    .to_string().chars()
    .rev().collect::<Vec<char>>()[10..].iter()
    .rev().collect::<String>();

    let save_map = MvRoom {
        keys: [vec![],vec![format!("{}",input_dir)]],
        width: img.width() as usize,
        doors: doors,
        notes,
        tiles,
        //id: format!("test_parsed"),// todo remove, redundant
        //doors,
        //notes: vec![],
        //containers,
    };

    Ok(save_map)
}
