use std::vec;
use std::collections::HashMap;
use std::fs::File;
use binary_rw::{self, BinaryWriter, Endian, FileStream};
use image::DynamicImage;
use priority_queue::PriorityQueue;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node<T>{
    value: T,
    freq: i32,
    binary: String,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}
impl<T> Node<T>{
    fn new(value: T, freq: i32)->Self{        
        Node{
        value,
        freq,
        binary: String::new(),
        left: None,
        right: None
        }
    }
}

fn construct_dictionaries(image: &mut DynamicImage, r: &mut HashMap<i16, i32>, g: &mut HashMap<i16, i32>,
    b: &mut HashMap<i16, i32>, height: u32, width: u32){
    
    let image_buffer = image.as_mut_rgb8().unwrap();

    for i in 0 .. height{
        for j in 0 .. width{
            let pixel = image_buffer[(j,i)];

            if let Some(mut val) = r.get_mut(&(pixel[0] as i16)){
                *val += 1;
            }
            else{
                r.insert(pixel[0] as i16, 1);
            }

            if let Some(mut val) = g.get_mut(&(pixel[1] as i16)){
                *val += 1;
            }
            else{
                g.insert(pixel[1] as i16, 1);
            }

            if let Some(mut val) = b.get_mut(&(pixel[2] as i16)){
                *val += 1;
            }
            else{
                b.insert(pixel[2] as i16, 1);
            }
        }
    }

}

fn build_huffman_tree(color: &mut HashMap<i16, i32>) -> Node<i16>{
    
    let mut pq: PriorityQueue<Node<i16>, i32> = PriorityQueue::new();

    for value in color.keys(){
        let freq = color.get(value).unwrap();
        let node: Node<i16> = Node::new(*value, *freq);
        pq.push(node, *freq);
    }
    for i in 0 .. color.len() - 1{
        let mut node: Node<i16> = Node::new(255, 0);
        let first_min: Node<i16> = pq.pop().unwrap().0;
        let second_min: Node<i16> = pq.pop().unwrap().0;
        node.freq = first_min.freq + second_min.freq;
        node.left = Some(Box::new(second_min));
        node.right = Some(Box::new(first_min));

        let pushed_freq = node.freq;
        pq.push(node, pushed_freq);
    }
    pq.pop().unwrap().0
}

fn dfs(node: &mut Node<i16>, binary: String, tree: &mut HashMap<i16, String>,
    freq_tree: &mut HashMap<i16, i32>, channel_size: &mut f32){

    if Some(&node).is_none(){
        return;
    }
    if node.value != 255{
        node.binary = binary.clone();
        tree.insert(node.value, binary.clone());
        
        *channel_size += (binary.len() * (*freq_tree.get(&node.value).unwrap()) as usize) as f32;
    }

    if let Some(ref mut left_node) = node.left {
        dfs(left_node, binary.clone() + "0", tree, freq_tree, channel_size);
    }

    // Check and recursively call `dfs` for the right child if it exists
    if let Some(ref mut right_node) = node.right {
        dfs(right_node, binary + "1", tree, freq_tree, channel_size);
    }
}

pub fn huffman_compress(image: &mut DynamicImage, tap_position: i32, init_seed: String, mut r: HashMap<i16, i32>, mut g: HashMap<i16, i32>,
    mut b: HashMap<i16, i32>, r_tree: &mut HashMap<i16, String>, g_tree: &mut HashMap<i16, String>, b_tree: &mut HashMap<i16, String>, height: u32, width:u32,) -> (f32, Node<i16>, Node<i16>, Node<i16>, Vec<String>){
    
    construct_dictionaries(image, &mut r, &mut g, &mut b, height, width);

    let mut root_red = build_huffman_tree(&mut r);
    let mut root_green = build_huffman_tree(&mut g);
    let mut root_blue = build_huffman_tree(&mut b);

    let mut r_channel: f32 = 0.0;
    let mut g_channel: f32 = 0.0;
    let mut b_channel: f32 = 0.0;
    
    r_tree.clear();
    g_tree.clear();
    b_tree.clear();
    
    dfs(&mut root_red, String::new(), r_tree, &mut r, &mut r_channel);
    dfs(&mut root_green, String::new(), g_tree, &mut g, &mut g_channel);
    dfs(&mut root_blue, String::new(), b_tree, &mut b, &mut b_channel);

    let image_channel_size = (height * width * 8) as f32;

    let red_ratio = (r_channel / image_channel_size) * 100.0;
    let green_ratio = (g_channel / image_channel_size) * 100.0;
    let blue_ratio = (b_channel / image_channel_size) * 100.0;
    let comp_ratio = (red_ratio + green_ratio + blue_ratio) / 3.0;

    let arrays: Vec<String> = pixel_encoding(image, height, width, r_tree, g_tree, b_tree);

    (comp_ratio, root_red, root_green, root_blue, arrays)

}

fn pixel_encoding(image: &mut DynamicImage, height: u32, width: u32,r_tree: &mut HashMap<i16, String>,
     g_tree: &mut HashMap<i16, String>, b_tree: &mut HashMap<i16, String>) -> Vec<String>{

    let mut red_string = String::new();
    let mut green_string = String::new();
    let mut blue_string = String::new();

    let image_buffer = image.as_mut_rgb8().unwrap();

    for i in 0 .. height{
        for j in 0 .. width{
            let pixel = image_buffer[(j, i)];

            red_string.push_str(r_tree.get(&(pixel[0] as i16)).unwrap());
            green_string.push_str(g_tree.get(&(pixel[1] as i16)).unwrap());
            blue_string.push_str(b_tree.get(&(pixel[2] as i16)).unwrap());
        }
    }

    let arrays = vec![red_string, green_string, blue_string];
    arrays
}

pub fn write_compressed_image(file_name: String, init_seed: String, tap_position: i32,
    mut red_root: Node<i16>, mut green_root: Node<i16>, mut blue_root: Node<i16>, width: u32,
    height: u32, rgb_channels: Vec<String>) -> Result<(), std::io::Error>{

    let mut file = File::create(file_name)?;
    let mut stream = FileStream::new(file);
    let mut writer = BinaryWriter::new(&mut stream,Endian::Little);
    
    writer.write_i32(tap_position);
    writer.write_string(init_seed);
    writer.write_u32(width);
    writer.write_u32(height);
    
    write_tree(&mut writer, &mut red_root);
    print_tree(& mut red_root);

    write_tree(&mut writer, &mut green_root);
    print_tree(&mut green_root);

    write_tree(&mut writer, &mut blue_root);
    print_tree(&mut blue_root);

    writer.write_usize(rgb_channels[0].len());
    writer.write_usize(rgb_channels[1].len());
    writer.write_usize(rgb_channels[2].len());

    write_channels(writer, rgb_channels);
    
    Ok(())
}


fn write_tree(writer: &mut BinaryWriter, node: &mut Node<i16>){
    if Some(&node).is_none(){
        return;
    }
    if node.value == 255{
        writer.write_i16(node.value);
    }
    else{
        writer.write_i16(node.value);
    }

    if let Some(ref mut left_node) = node.left {
        write_tree(writer, left_node);
    }

    // Check and recursively call `dfs` for the right child if it exists
    if let Some(ref mut right_node) = node.right {
        write_tree(writer, right_node);
    }
    
    
}

fn write_channels(mut writer: BinaryWriter, channels: Vec<String>){

    for channel in channels{
        let bit_length: usize = channel.len();
        let mut bytes: Vec<u8> = vec![0;(bit_length + 7) / 8];

        for i in 0 .. bit_length{
            if channel.chars().nth(i).unwrap() == '1'{
                let byte_index = i / 8;
                let bit_offset = i % 8;
                bytes[byte_index] |= (1 << (7 - bit_offset)) as u8; 
            }
        }

        writer.write_bytes(bytes);
    }
}

fn print_tree(node: &mut Node<i16>){

    if Some(&node).is_none(){
        return;
    }
    if node.value == 255{
        println!("node freq: {}", node.freq);
    }
    else{
        println!("leaf node");
        println!("node value: {}", node.value);
        println!("node freq: {}", node.freq);
        println!("end of leaf node");
    }

    if let Some(ref mut left_node) = node.left {
        print_tree(left_node);
    }

    
    if let Some(ref mut right_node) = node.right {
        print_tree(right_node);
    }
}