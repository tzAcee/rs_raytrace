use ggez::event;
use ggez::graphics::{self, Image};
use ggez::{Context, GameResult};
use rgb::RGBA8;

mod ray;
use ray::*;

use vec3D::*;

fn hit_sphere(center: Vec3D, radius: f32, r: &Ray) -> bool {
    let oc = *r.origin() - center;
    let a = r.direction().dot(*r.direction());
    let b = 2.0 * oc.dot(*r.direction());
    let c = oc.dot(oc) - radius as f64*radius as f64;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn ray_color(r: Ray) -> Vec3D {
    if hit_sphere(Vec3D{x: 0.0, y:0.0, z:-1.0},0.5, &r) {
        return Vec3D{x:1.0, y:0.0, z:0.0};
    }

    let unit_direction = r.direction().unit();
    let t = 0.5*(unit_direction.y + 1.0);
    return Vec3D{x:1.0,y:1.0,z:1.0}*(1.0-t)+Vec3D{x:0.5,y:0.7,z:1.0}*t;
}

pub struct WindowState {
    aspect_ratio: f32,
    pixels: Vec<u8>,
    control_pixels: Vec<u8>
}

impl WindowState {
    pub fn new(ar: f32) -> GameResult<WindowState> {
        let s = WindowState {
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
        // CAMERA
        let viewport_height:f64 = 2.0;
        let viewport_width = self.aspect_ratio as f64 * viewport_height;
        let focal_length = 1.0;
    
        let origin = Vec3D{x:0.0, y:0.0, z:0.0};
        let horizontal = Vec3D{x:viewport_width, y:0.0, z:0.0};
        let vertical = Vec3D{x:0.0, y:viewport_height, z:0.0};
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3D{x:0.0, y:0.0, z:focal_length};

        let mut rgb_pixels : Vec<rgb::RGBA8> = Vec::<rgb::RGBA8>::new();
        let (width, height) = graphics::size(ctx);

        for y in 0..height as u16 {
            for x in 0..width as u16 {
                let u = x as f64 / (width as f64-1.0);
                let v =  y as f64 / (height as f64-1.0);
                let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
                let pixel_color = ray_color(r);

                let ir = (pixel_color.x*255.999) as u8;
                let ig = (pixel_color.y*255.999) as u8;
                let ib = (pixel_color.z*255.999) as u8;

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