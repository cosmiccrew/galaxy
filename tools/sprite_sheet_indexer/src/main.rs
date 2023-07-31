use std::{collections::BTreeMap, fs, path::PathBuf};

use show_image::create_window;
use text_io::read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct SpriteSheetMeta(BTreeMap<String, Vec<(u32, u32)>>);

impl SpriteSheetMeta {
    fn new() -> Self {
        Self(BTreeMap::new())
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WRITE IN FORMAT: [colour, symbol, type, lightness]");

    let input_file = PathBuf::from("assets/ui/icons/prompts_16x16.png");

    let input_file_name = input_file.clone();
    let input_file_name = input_file_name.file_stem().unwrap();

    let ron_file = format!("{}.spritesheet.ron", input_file_name.to_str().unwrap());

    let existing: SpriteSheetMeta =
        ron::de::from_bytes(fs::read(ron_file.clone()).unwrap_or(vec![]).as_slice())
            .unwrap_or(SpriteSheetMeta::new());

    let mut sprite_sheet_raw = image::open(input_file)?;
    // .crop(0, 0, 2*16, 2*16);

    let overlay = image::open("assets/ui/icons/overlay_16x16.png")?;

    let always_window = create_window("sprite sheet view", Default::default())?;

    let window = create_window("sprite", Default::default())?;

    let width = sprite_sheet_raw.clone().width();
    let height = sprite_sheet_raw.clone().height();

    let mut hashmap = existing.0.clone();

    let mut current: Vec<(u32, u32)> = vec![];

    let mut previous = String::new();

    always_window.set_image("image-001", sprite_sheet_raw.clone())?;

    let mut skip = false;

    'outer: for y in 0..(height / 16) {
        for x in 0..(width / 16) {
            if !existing
                .0
                .iter()
                .any(|(_, value_iter)| value_iter.iter().any(|value| value == &(x, y)))
            {
                if skip {
                    print!("Skips completed. Next choosable index: ({x},{y}).");
                    println!();
                    skip = false;
                }

                let mut new_spritesheet_image = sprite_sheet_raw.clone();

                image::imageops::overlay(
                    &mut new_spritesheet_image,
                    &overlay,
                    (x * 16) as i64,
                    (y * 16) as i64,
                );

                always_window.set_image("image-001", new_spritesheet_image)?;

                let image = sprite_sheet_raw.crop(x * 16, y * 16, 16, 16);

                window.set_image("image-001", image)?;

                let mut name: String = read!("{}\n");

                while hashmap.contains_key(&name.replace(' ', "_")) {
                    println!("An indexed sprite with the name: {name} already exists!");
                    name = read!("{}\n");
                }

                current.push((x, y));

                if name == "[EDIT]" && x != 0 && y != 0 {
                    let old = hashmap.remove_entry(&previous).unwrap();

                    print!("enter new name for previous: ");
                    let new: String = read!("{}\n");

                    hashmap.insert(new.replace(' ', "_"), old.1);

                    name = read!("{}\n");
                }
                // else if name == "[BACK]" {
                //     if x == 0 {
                //         y -= 1;
                //         x = (height / 16) - 1;
                //     } else if x == 1 {
                //         y -= 1;
                //         x = (height / 16);
                //     } else {
                //         x -= 2;
                //     }
                //     println!("going back");
                //     continue;
                // }

                if name == "[END]" {
                    println!("End command ran: program terminating.");
                    break 'outer;
                } else if name == "[SKIP]" {
                    println!("Image skipped!");
                    continue;
                }

                name = name.replace(' ', "_");

                if !(name.ends_with('+') || name.is_empty()) {
                    hashmap.insert(name.clone(), current.clone());
                    current.clear();
                }

                previous = name.clone();
            } else {
                print!("Skipped ({},{}) | ", x, y);
                skip = true;
            }
        }
    }

    let sprite_sheet_meta = SpriteSheetMeta(hashmap);

    let output_file = fs::File::create(ron_file)?;

    ron::ser::to_writer_pretty(&output_file, &sprite_sheet_meta, Default::default())?;

    Ok(())
}
