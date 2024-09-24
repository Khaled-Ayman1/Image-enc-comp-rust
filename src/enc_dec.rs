use ggez::graphics::Image;
use image::{DynamicImage, GenericImageView};

use crate::MainState;

const KEY_SIZE: usize = 8;

pub fn key_generation(tap_position : usize, seed : &mut Vec<char>, rgb_keys: &mut Vec<Vec<char>>){
   
    let bit_size : usize = seed.len();
    
    for k in 0 .. rgb_keys.len(){   
        
        let mut key_string: Vec<char> = vec![' '; KEY_SIZE];

        for i in 0 .. KEY_SIZE{            
            let shift_out : char = seed[0];
            let res : char = (seed[(bit_size - tap_position) - 1] as u8 ^ shift_out as u8) as char;
   
            for j in 1 .. bit_size{
                seed[j - 1] = seed[j];
            }
   
            seed[bit_size - 1] = res;
            key_string[i] = res;
        }

        rgb_keys[k] = key_string; 
    }
} 

pub fn lfsr(image_matrix : &mut MainState, rgb_keys : &mut Vec<Vec<char>>, tap_position : usize, init_seed : String, width: u32, height: u32) -> DynamicImage{
    let mut seed : Vec<char> = init_seed.chars().collect();


    for (row, col, mut pixel) in image_matrix.image.pixels(){

        key_generation(tap_position, &mut seed, rgb_keys);

        let red_key = &mut rgb_keys[0];
        pixel[0] = (pixel[0] ^ convert_byte(red_key)) as u8;

        let green_key = &mut rgb_keys[1];
        pixel[1] = (pixel[1] ^ convert_byte(green_key)) as u8;

        let blue_key = &mut rgb_keys[2];
        pixel[2] = (pixel[2] ^ convert_byte(blue_key)) as u8;
    }

    image_matrix.image.clone()

}

fn convert_byte(key : &mut Vec<char>) -> u8{
    let mut res = 0;
    for i in 0 .. key.len(){
        res = ((res << 1) | (key[i] as u8)) as u8;
    }
    res
}