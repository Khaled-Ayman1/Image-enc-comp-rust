use std::collections::HashMap;
use std::path::PathBuf;
mod enc_dec;
mod comp;

use ggez::glam::{vec2, Vec2};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image, ImageFormat, Rect};
use ggez::event::{self, EventHandler};
use image::{imageops, open};
use rfd::FileDialog;
use image::{io::Reader as ImageReader, DynamicImage, ImageResult, RgbImage};

struct MainState {
    image: DynamicImage, 
    resized_img: DynamicImage,
    enc_img: DynamicImage,
    resized_enc: DynamicImage,
    is_enc: bool,
    comp_img: DynamicImage,
    resized_comp: DynamicImage,
    is_comp: bool
}

// buttons x, y coordinates
const LOAD_IMG_BUTTON: (f32, f32) = (175.0, 350.0);
const ENCRYPT_BUTTON: (f32, f32) = (510.0, 350.0);
const COMPRESS_BUTTON: (f32, f32) = (510.0, 100.0);

impl MainState {
    fn new() -> GameResult<MainState> {
        // code is not safe, run-time error is possible
        let path = get_path_from_file_system("Select Image");
                                
        let s = path 
        .to_str()
        .unwrap();
    
        let iamge_state = Self::get_image_state(s);
        
        Ok(iamge_state)
    }
    //this function take the new state of the new image 
    // and make the Main state equla to this new state
    fn change_state(&mut self , new_state : MainState  ){
        *self = new_state;
    }
    // This function processes the image that is obtained from the file system
    //then create a new state of this processed image and return this state
    fn get_image_state(path : &str)->MainState{
        let img: Result<ImageReader<std::io::BufReader<std::fs::File>>, std::io::Error> = ImageReader::open(path);

        let i1 = img.unwrap().decode().unwrap();

        let resized = i1.resize(200, 200, image::imageops::FilterType::Gaussian);

        let enc = i1.clone();

        let resized_enc = enc.resize(200, 200, image::imageops::FilterType::Gaussian);
        
        let comp = i1.clone();

        let resized_comp = comp.resize(200, 200, image::imageops::FilterType::Gaussian);

        let image_state = MainState { 
            image: i1,
            resized_img: resized,
            enc_img: enc,
            resized_enc: resized_enc,
            is_enc: false,
            comp_img: comp,
            resized_comp: resized_comp,
            is_comp: false
        };
        return  image_state;
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    // Draw the Screen and set its Color and draw its elements
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        // text on the button used to load image
        let load_img_text = graphics::Text::new("load image");
        // text on the button used to Encrypt image
        let enc_text = graphics::Text::new("encrypt");
        
        let comp_text = graphics::Text::new("compress");

        // create Load button and set its properties
        let load_button = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            //define button x, y coordinates in x ,y plane and also define its width and height
            Rect::new(LOAD_IMG_BUTTON.0, LOAD_IMG_BUTTON.1, 100.0, 25.0), 
            Color::RED,
        )?;
        // create Encrypt button and set its properties
        let enc_button = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            //define button x, y coordinates in x ,y plane and also define its width and height
            Rect::new(ENCRYPT_BUTTON.0,ENCRYPT_BUTTON.1, 100.0, 25.0), 
            Color::RED,
        )?;

        let comp_button = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            //define button x, y coordinates in x ,y plane and also define its width and height
            Rect::new(COMPRESS_BUTTON.0,COMPRESS_BUTTON.1, 100.0, 25.0), 
            Color::RED,
        )?;

        
        // encrypt image : (show the image on the screen after Encryption)
        if self.is_enc{
            let enced = self.resized_enc.to_rgba8();
            let (width, height) = (enced.width(), enced.height());

            let img = Image::from_pixels(ctx, enced.as_ref(),
            ImageFormat::Rgba8UnormSrgb,
            width,
            height,);
            canvas.draw(&img, Vec2::new(460.0,120.0));
        }

        if self.is_comp{
            let compd = self.resized_comp.to_rgb8();
            let (width, height) = (compd.width(), compd.height());

            let img = Image::from_pixels(ctx, &compd.as_ref(),
            ImageFormat::Rgba8UnormSrgb,
            width,
            height,);
            canvas.draw(&img, Vec2::new(460.0,120.0));
        }

        // from dynamic image to Image
        let rgba8 = self.resized_img.to_rgba8();
        let (width, height) = (rgba8.width(), rgba8.height());

        let img = Image::from_pixels(ctx, rgba8.as_ref(),
        ImageFormat::Rgba8UnormSrgb,
        width,
        height,);

                            // draw on screen
        //draw the image
        canvas.draw(&img, Vec2::new(120.0, 120.0));
        //draw the buuton to load image
        canvas.draw(&load_button, Vec2::new(0.0, 0.0));
        // draw the text on the Load _button
        canvas.draw(&load_img_text, Vec2::new(LOAD_IMG_BUTTON.0+5.0 , LOAD_IMG_BUTTON.1+5.0 ));
        //draw the buuton to Encrypt image
        canvas.draw(&enc_button, Vec2::new(0.0, 0.0));
        // draw the text on the Enc _button
        canvas.draw(&enc_text, Vec2::new(ENCRYPT_BUTTON.0 + 20.0, ENCRYPT_BUTTON.1 + 5.0));
        
        canvas.draw(&comp_button, Vec2::new(0.0, 0.0));
        canvas.draw(&comp_text, Vec2::new(COMPRESS_BUTTON.0 + 20.0, COMPRESS_BUTTON.1 + 5.0));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,   // x ,y is coordinates of clicking the mouse
        ) -> Result<(), ggez::GameError> {
        
        // if the coordinates of clicking the mouse is inside the border of the load_image button
        // so we will load the image
        if _x >= LOAD_IMG_BUTTON.0 && _x <= LOAD_IMG_BUTTON.0 + 100.0{
            if _y >= LOAD_IMG_BUTTON.1 && _y <= LOAD_IMG_BUTTON.1 + 25.0{
                
                let path = get_path_from_file_system("Select Image");
                        
                let s = path
                .to_str()
                .unwrap();
                
                let new_image_state = Self::get_image_state(s);
                self.change_state(new_image_state);
            } 
        }

        // encrypt : (this function is responsbile for Encryption Process)
        if _x >= ENCRYPT_BUTTON.0 && _x <= ENCRYPT_BUTTON.0 + 100.0{
            if _y >= ENCRYPT_BUTTON.1 && _y <= ENCRYPT_BUTTON.1 + 25.0{
                self.is_enc = true;

                // TODO: image encryption
                // remove following line
                let width = self.image.width();
                let height = self.image.height();
                let dimensions = width * height;
                let mut rgb_keys: Vec<Vec<char>> = vec![vec![' '; dimensions as usize], vec![' '; dimensions as usize], vec![' '; dimensions as usize]];

                enc_dec::lfsr(self, &mut rgb_keys, 2, String::from("111100011")
                , width, height);
                self.resized_enc = self.enc_img.resize(200, 200, image::imageops::FilterType::Gaussian);
            }
        }

        // compression
        // TODO: compression & decompression
        if _x >= COMPRESS_BUTTON.0 && _x <= COMPRESS_BUTTON.0 + 100.0{
            if _y >= COMPRESS_BUTTON.1 && _y <= COMPRESS_BUTTON.1 + 25.0{
                
                self.is_comp = true;

                let width = self.image.width();
                let height = self.image.height();

                let mut r: HashMap<u8, i32> = HashMap::new();
                let mut g: HashMap<u8, i32> = HashMap::new();
                let mut b: HashMap<u8, i32> = HashMap::new();

                let mut r_tree: HashMap<u8, String> = HashMap::new();
                let mut g_tree: HashMap<u8, String> = HashMap::new();
                let mut b_tree: HashMap<u8, String> = HashMap::new();


            }
        }

        Ok(())
    }
    
}
// this function open the file system to enanle us choosing the image that we want
// and  then return teh path of this image
fn get_path_from_file_system(title : &str)->PathBuf{
    FileDialog::new()
    .set_title(title)
    .pick_file().unwrap()
}


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}