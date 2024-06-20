use ggez::glam::Vec2;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image, ImageFormat, Rect};
use ggez::event::{self, EventHandler};
use image::imageops;
use rfd::FileDialog;
use image::{io::Reader as ImageReader, DynamicImage, ImageResult, RgbImage};

struct MainState {
    image: DynamicImage, 
    resized_img: DynamicImage,
    enc_img: DynamicImage,
    resized_enc: DynamicImage,
    is_enc: bool,
}

// buttons x, y coordinates
const LOAD_IMG_BUTTON: (f32, f32) = (200.0, 360.0);
const ENCRYPT_BUTTON: (f32, f32) = (540.0, 360.0);

impl MainState {
    fn new() -> GameResult<MainState> {
        // code is not safe, run-time error is possible
        // must select an image
        let path = FileDialog::new()
                                    .set_title("Select Image")
                                    .pick_file().unwrap();
                                
        let s = path
        .to_str()
        .unwrap();

        let img = ImageReader::open(s);

        let i1 = img.unwrap().decode().unwrap();

        let resized = i1.resize(200, 200, image::imageops::FilterType::Gaussian);

        let enc = i1.clone();

        let resized_enc = enc.resize(200, 200, image::imageops::FilterType::Gaussian);

        let s = MainState { 
            image: i1,
            resized_img: resized,
            enc_img: enc,
            resized_enc: resized_enc,
            is_enc: false,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        // buttons and text
        let load_img_text = graphics::Text::new("load image");

        let enc_text = graphics::Text::new("encrypt");

        let button = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(LOAD_IMG_BUTTON.0, LOAD_IMG_BUTTON.1, 40.0, 15.0), 
            Color::WHITE,
        )?;

        let enc_button = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(ENCRYPT_BUTTON.0,ENCRYPT_BUTTON.1, 40.0, 15.0), 
            Color::WHITE,
        )?;

        // encrypt image
        if self.is_enc{
            let enced = self.resized_enc.to_rgba8();
            let (width, height) = (enced.width(), enced.height());

            let img = Image::from_pixels(ctx, enced.as_ref(),
            ImageFormat::Rgba8UnormSrgb,
            width,
            height,);

            canvas.draw(&img, Vec2::new(460.0, 120.0));
        }

        // from dynamic image to Image
        let rgba8 = self.resized_img.to_rgba8();
        let (width, height) = (rgba8.width(), rgba8.height());

        let img = Image::from_pixels(ctx, rgba8.as_ref(),
        ImageFormat::Rgba8UnormSrgb,
        width,
        height,);

        // draw on screen
        canvas.draw(&img, Vec2::new(120.0, 120.0));

        canvas.draw(&button, Vec2::new(0.0, 0.0));

        canvas.draw(&load_img_text, Vec2::new(LOAD_IMG_BUTTON.0 - 20.0, LOAD_IMG_BUTTON.1 + 20.0));

        canvas.draw(&enc_button, Vec2::new(0.0, 0.0));

        canvas.draw(&enc_text, Vec2::new(ENCRYPT_BUTTON.0 - 10.0, ENCRYPT_BUTTON.1 + 20.0));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), ggez::GameError> {
        
        // load image
        if _x >= LOAD_IMG_BUTTON.0 && _x <= LOAD_IMG_BUTTON.0 + 40.0{
            if _y >= LOAD_IMG_BUTTON.1 && _y <= LOAD_IMG_BUTTON.1 + 15.0{
                let path = FileDialog::new()
                                    .set_title("Select Image")
                                    .pick_file().unwrap();
                                
                let s = path
                .to_str()
                .unwrap();

                let img = ImageReader::open(s);

                let i1 = img.unwrap().decode().unwrap();

                self.image = i1.clone();

                let resized = self.image.resize(200, 200, image::imageops::FilterType::Gaussian);

                let enc = i1;

                let resized_enc = enc.resize(200, 200, image::imageops::FilterType::Gaussian);

                self.enc_img = enc;

                self.resized_enc = resized_enc;

                self.resized_img = resized;

                self.is_enc = false;
            } 
        }

        // encrypt
        if _x >= ENCRYPT_BUTTON.0 && _x <= ENCRYPT_BUTTON.0 + 40.0{
            if _y >= ENCRYPT_BUTTON.1 && _y <= ENCRYPT_BUTTON.1 + 15.0{
                self.is_enc = true;

                // TODO: image encryption
                // remove following line
                self.resized_enc = self.enc_img.grayscale().resize(200, 200, imageops::FilterType::Gaussian);
            }
        }

        // compression
        // TODO: compression & decompression

        Ok(())
    }

    
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}