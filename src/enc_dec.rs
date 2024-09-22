use crate::MainState;


//static mut RGB_KEYS: Vec<Vec<char>> = vec![Vec::new(), Vec::new(), Vec::new()];
static KEY_SIZE: usize = 8;

pub fn key_generation(tap_position : usize, seed : &mut Vec<char>, keys: &mut Vec<Vec<char>>){
   
    let bit_size : usize = seed.len();
    
    for k in 0 .. keys.len(){   
        
        let mut key_string: Vec<char> = Vec::with_capacity(KEY_SIZE);

        for i in 0 .. KEY_SIZE{            
            let shift_out : char = seed[0];
            let res : char = (seed[(bit_size - tap_position) - 1] as u8 ^ shift_out as u8) as char;
   
            for j in 1 .. bit_size{
                seed[j - 1] = seed[j];
            }
   
            seed[bit_size - 1] = res;
            key_string[i] = res;
        }

        keys[k] = key_string; 
    }
} 

pub fn lfsr(image_matrix : MainState, tap_position : usize, init_seed : &mut str, width: i32, height: i32){
    let seed : Vec<char> = init_seed.chars().collect();

    for row in 0 .. height{
        for col in 0 .. width{
            
        }
    }
}