use ggez::event;
use ggez::graphics::{self, Image};
use ggez::{Context, GameResult};
use rgb::RGBA8;

pub struct WindowState {
    width: f32,
    height: f32,
    aspect_ratio: f32,
    pixels: Vec<u8>,
    control_pixels: Vec<u8>
}

impl WindowState {
    pub fn new(w: f32, h: f32, ar: f32) -> GameResult<WindowState> {
        let s = WindowState {
            width: w,
            height: h,
            aspect_ratio: ar,
            pixels: Vec::<u8>::new(),
            control_pixels: Vec::<u8>::new()
        };
        Ok(s)
    }

    fn pixel_has_changed(&mut self) -> bool {
        if self.pixels == self.control_pixels {
            return false;
        } 
        self.control_pixels = self.pixels.to_owned();
        return true;
    }

    fn refresh_pixel(&mut self, ctx: &Context) {
        let mut rgb_pixels : Vec<rgb::RGBA8> = Vec::<rgb::RGBA8>::new();
        let (width, height) = graphics::size(ctx);

        for y in 0..height as u16 {
            for x in 0..width as u16 {
                let r = x as f32 / (width-1.0);
                let g =  y as f32 / (height-1.0);
                let b = 0.25;

                let ir = (r*255.999) as u8;
                let ig = (g*255.999) as u8;
                let ib = (b*255.999) as u8;

                rgb_pixels.push(RGBA8 { r: ir, g: ig,b: ib, a: 255});
            }
        }

        use rgb::ComponentBytes; 
        self.pixels = rgb_pixels.as_bytes().to_vec();
    }
}

impl event::EventHandler<ggez::GameError> for WindowState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.refresh_pixel(_ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.pixel_has_changed() {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let (width, height) = graphics::size(ctx);
        let draw_img = Image::from_rgba8(ctx, width as u16, height as u16, &self.pixels)?;
        
        graphics::draw(ctx, &draw_img, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        }
        Ok(())
    }
}