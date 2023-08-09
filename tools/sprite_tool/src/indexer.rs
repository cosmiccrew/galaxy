use anyhow::{anyhow, Result};
use show_image::create_window;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{collections::BTreeMap, fs};
use text_io::read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
struct SpriteSheetMeta(BTreeMap<String, Vec<(u32, u32)>>);

impl SpriteSheetMeta {
    fn new() -> Self {
        Self::default()
    }
}

pub fn indexer(input_file: PathBuf, output_file: Option<PathBuf>) -> Result<()> {
    let output_path = output_path(&input_file, output_file)?;

    println!("Enter names in this format: \"colour/product variant/type symbol/item colour_scheme\" - for example, \"red b button light\" or \"switch screenshot button dark\"");

    let existing: SpriteSheetMeta =
        ron::de::from_bytes(fs::read(output_path.clone()).unwrap_or(vec![]).as_slice())
            .unwrap_or(SpriteSheetMeta::new());

    let mut sprite_sheet_raw = image::open(input_file)?;

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

    let mut x = 0u32;
    let mut y = 0u32;

    let sprite_size = 16u32;
    let sprite_spacing = 0u32;
    let sprite_full_size = sprite_size + sprite_spacing;

    let x_max = width
        .checked_div(sprite_size + sprite_spacing)
        .ok_or(anyhow!("invalid sprite x height!"))?;

    let y_max = height
        .checked_div(sprite_size + sprite_spacing)
        .ok_or(anyhow!("invalid sprite y height!"))?;

    'y_loop: while y < y_max {
        y += 1;

        let y_sprite_size = y * sprite_size;
        let y_skip_size = y * (sprite_size + sprite_spacing);

        while x < x_max {
            x += sprite_full_size;

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
                    (x * sprite_size) as i64,
                    (y * sprite_size) as i64,
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
                    break 'y_loop;
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

        x = 0;
    }

    'outer: for y in 0..y_max {
        for x in 0..x_max {
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
                    (x * sprite_size) as i64,
                    (y * sprite_size) as i64,
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

    let output_path = fs::File::create(output_path)?;

    ron::ser::to_writer_pretty(&output_path, &sprite_sheet_meta, Default::default())?;

    Ok(())
}

fn output_path(input_file: &Path, output_file: Option<PathBuf>) -> Result<PathBuf> {
    Ok(if let Some(file) = output_file {
        file
    } else {
        let file_stem = &input_file.file_stem().ok_or(anyhow!("not a file!"))?;

        input_file
            .parent()
            .ok_or(anyhow::anyhow!(
                "no parent directory of the input file found!"
            ))?
            .join(PathBuf::from_str(&format!(
                "{}.spritesheet.ron",
                file_stem
                    .to_str()
                    .ok_or(anyhow::anyhow!("could not convert file_name to a str."))?
            ))?)
    })
}
