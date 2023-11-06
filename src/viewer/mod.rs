use std::env;
use std::fs::{metadata, read_dir};

use image::{ImageBuffer, Rgb};

const BYTE_MAP_SIZE: usize = 256; // 8 bits = 1 byte = 2 hex
const COLOR_RGBA_WHITE: usize = 255;
// const COLOR_RGBA_BLACK: usize = 0;

pub fn view() {
    let args: Vec<String> = env::args().collect();

    let md_arg = metadata(&args[1]).expect("Error: The argument provided does not correspond to a valid path");
    if md_arg.is_dir() {
        println!("The argument provided is a directory");
        
        let paths_iterator = read_dir(&args[1]).unwrap();
        for path in paths_iterator {
            let filename = path.unwrap().path().display().to_string();
            println!("Processing {}", filename);
            view_file(&filename);
        }
    } else if md_arg.is_file() {
        println!("The argument provided is a file");
        println!("Processing {}", &args[1]);
        view_file(&args[1]);
    } else {
        println!("The argument provided is not known. Maybe it is neither a directory or a file");
    }
}

fn view_file(filename: &str) {
    let file: Vec<usize> = std::fs::read(filename).unwrap().iter().map(|&a| a as usize).collect::<Vec<usize>>();
    let mut file_iter = file.iter();
    
    let mut byte_map: Vec<f64> = vec![0.0; BYTE_MAP_SIZE * BYTE_MAP_SIZE];

    let mut current_byte: usize;
    match file_iter.next() {
        Some(&first_byte) => current_byte = first_byte,
        None => panic!("Error when accessing the file bytes")
    }
    loop {
        match file_iter.next() {
            Some(&next_byte) => {
                let idx: usize = BYTE_MAP_SIZE * next_byte + current_byte;
                byte_map[idx] += 1.;

                current_byte = next_byte;
            }
            None => {
                break;
            }
        }
    }

    let max_value = *byte_map.iter().max_by(|a,b| a.total_cmp(b)).unwrap();
    byte_map = byte_map.iter().map(|&a| a.log10() / max_value.log10() * (COLOR_RGBA_WHITE as f64)).collect();
    
    // Create the output image
    let mut img = ImageBuffer::new(BYTE_MAP_SIZE as u32, BYTE_MAP_SIZE as u32);
    for x in 0..BYTE_MAP_SIZE {
        for y in 0..BYTE_MAP_SIZE {
            let x_u32 = x as u32;
            let y_u32 = y as u32;
            let idx_byte_map = BYTE_MAP_SIZE * y + x;
            let pixel_rgb_value = byte_map[idx_byte_map] as u8;
            img.put_pixel(x_u32, y_u32, Rgb([pixel_rgb_value, pixel_rgb_value, pixel_rgb_value]));
        }
    }
    let filename_splitted: Vec<_> = filename.split(|c| c == '/' || c == '.').collect();
    let output_filename: String = format!("{}/output_{}.png", &filename_splitted[0], &filename_splitted[1]);
    img.save(&output_filename).unwrap();
}