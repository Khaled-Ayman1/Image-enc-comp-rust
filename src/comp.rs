use std::{arch::x86_64::_SIDD_LEAST_SIGNIFICANT, collections::HashMap};
use ggez::winit::dpi::validate_scale_factor;
use image::DynamicImage;
use priority_queue::PriorityQueue;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node<T>{
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

fn construct_dictionaries(image: &mut DynamicImage, r: &mut HashMap<u8, i32>, g: &mut HashMap<u8, i32>,
    b: &mut HashMap<u8, i32>, height: u32, width: u32){
    
    let image_buffer = image.as_mut_rgb8().unwrap();

    for i in 0 .. height{
        for j in 0 .. width{
            let pixel = image_buffer[(j,i)];

            if let Some(mut val) = r.get_mut(&pixel[0]){
                *val += 1;
            }
            else{
                r.insert(pixel[0], 1);
            }

            if let Some(mut val) = g.get_mut(&pixel[1]){
                *val += 1;
            }
            else{
                g.insert(pixel[1], 1);
            }

            if let Some(mut val) = b.get_mut(&pixel[2]){
                *val += 1;
            }
            else{
                b.insert(pixel[2], 1);
            }
        }
    }

}

fn build_huffman_tree(color: &mut HashMap<u8, i32>) -> Node<u8>{
    
    let mut pq: PriorityQueue<Node<u8>, i32> = PriorityQueue::new();

    for value in color.keys(){
        let freq = color.get(value).unwrap();
        let node: Node<u8> = Node::new(*value, *freq);
        pq.push(node, *freq);
    }
    for i in 0 .. color.len() - 1{
        let mut node: Node<u8> = Node::new(255, 0);
        let first_min: Node<u8> = pq.pop().unwrap().0;
        let second_min: Node<u8> = pq.pop().unwrap().0;
        node.freq = first_min.freq + second_min.freq;
        node.left = Some(Box::new(second_min));
        node.right = Some(Box::new(first_min));

        let pushed_freq = node.freq;
        pq.push(node, pushed_freq);
    }
    pq.pop().unwrap().0
}

fn dfs(node: &mut Node<u8>, binary: String, tree: &mut HashMap<u8, String>,
    freq_tree: &mut HashMap<u8, i32>, channel_size: &mut f32){

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

fn huffman_compress(mut image:DynamicImage, tap_position: i32, init_seed: String, mut r: HashMap<u8, i32>, mut g: HashMap<u8, i32>,
    mut b: HashMap<u8, i32>, r_tree: &mut HashMap<u8, String>, g_tree: &mut HashMap<u8, String>, b_tree: &mut HashMap<u8, String>, height: u32, width:u32,) -> (f32, Node<u8>, Node<u8>, Node<u8>, Vec<String>){
    
    construct_dictionaries(&mut image, &mut r, &mut g, &mut b, height, width);

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

fn pixel_encoding(mut image: DynamicImage, height: u32, width: u32,r_tree: &mut HashMap<u8, String>,
     g_tree: &mut HashMap<u8, String>, b_tree: &mut HashMap<u8, String>) -> Vec<String>{

    let mut red_string = String::new();
    let mut green_string = String::new();
    let mut blue_string = String::new();

    let image_buffer = image.as_mut_rgb8().unwrap();

    for i in 0 .. height{
        for j in 0 .. width{
            let pixel = image_buffer[(width, height)];

            red_string.push_str(r_tree.get(&pixel[0]).unwrap());
            green_string.push_str(g_tree.get(&pixel[1]).unwrap());
            blue_string.push_str(b_tree.get(&pixel[2]).unwrap());
        }
    }

    let arrays = vec![red_string, green_string, blue_string];
    arrays
}