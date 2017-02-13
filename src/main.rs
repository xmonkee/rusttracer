#[macro_use]
extern crate bmp;
extern crate ray;
extern crate rand;
use ray::vec3::{Vec3, Ray};
use ray::scenes::*;
use ray::{Hitable, HitRecord};
use rand::{ThreadRng, Rng};
use bmp::{Image, Pixel};

/* Actual canval pixel size */
const NX:u32 = 400;
const NY:u32 = 200;

/* Virtual canval size limits 
 * X axis will go from -VX to VX and so on */
const VX:f32 = 2.0;
const VY:f32 = 1.0;
const VZ:f32 = 1.0;

static BLUE:Vec3 = Vec3 {e: [0.5f32, 0.7f32, 1.0f32]};
static WHITE:Vec3 = Vec3 {e: [1f32, 1f32, 1f32]};

static LOWER_CORNER:Vec3 = Vec3 {e: [-VX, -VY, -VZ]};
static ORIGIN:Vec3 = Vec3 {e: [0f32, 0f32, 0f32]};
static VERTICAL:Vec3 = Vec3 {e: [0f32, 2f32, 0f32]};
static HORIZONTAL:Vec3 = Vec3 {e: [4f32, 0f32, 0f32]};

fn main() {
    let mut img: Image = Image::new(NX, NY);
    //let scene = blank::Blank::new();
    let sphere = sphere::Sphere::new(
        Vec3::new(0f32, 0f32, -1f32), 
        0.5f32,
    );
    let sphere2 = sphere::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    );
    let mut scene = list::List::new();
    scene.add(sphere);
    scene.add(sphere2);

    let mut camera = Camera::new(scene);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, camera.color(x, y).to_pixel())
    }
    let _ = img.save("img.bmp");
}

struct Camera {
    rng: ThreadRng,
    scene: Box<Hitable>,
}

impl Camera {
    fn new<H: Hitable + 'static>(scene: H) -> Camera {
        Camera{
            rng: rand::thread_rng(),
            scene: Box::new(scene),
        }
    }

    fn random_in_unit_sphere(&mut self) -> Vec3 {
        let mut p = Vec3::new(1f32, 1f32, 1f32);
        while p.lensq() >= 1.0 {
            p = Vec3::new(
                self.rng.gen::<f32>(),
                self.rng.gen::<f32>(),
                self.rng.gen::<f32>(),
            )
        }
        return p;
    }

    fn ray_from_origin(&mut self, x: u32, y:u32) -> Ray {
        let xrand = self.rng.gen::<f32>() - 0.5;
        let yrand = self.rng.gen::<f32>() - 0.5;
        let u = HORIZONTAL * (x as f32 + xrand) / (NX as f32);
        let v = VERTICAL * ((NY-y) as f32 + yrand) / (NY as f32);
        Ray::new(ORIGIN, LOWER_CORNER - ORIGIN + u + v)
    }

    fn color(&mut self, x: u32, y: u32) -> Vec3 {
        let mut c = Vec3::new(0f32, 0f32, 0f32);

        let ns = 10;
        for _ in 0..ns {
            let ray = &self.ray_from_origin(x, y);
            c = c + self.color1(ray);
        }
        c / ns as f32
    }

    fn color1(&mut self, ray : &Ray) -> Vec3 {
        let hit = self.scene.hit(0.0, 1000.0, ray);
        match hit {
            Some(t) => {
                let target = t.p + t.normal + self.random_in_unit_sphere();
                self.color1(&Ray::new(t.p, target-t.p)) * 0.5 
            },
            None => {
                let y_cord = ray.direction().y();
                let t = 0.5 * VY * (y_cord + VY);
                BLUE * (1.0 - t) + WHITE * t
            }
        }
    }
}



trait AsPixel {
    fn to_pixel(&self) -> Pixel;
}

impl AsPixel for Vec3 {
    fn to_pixel(&self) -> Pixel {
        Pixel {
            r: (self.r()*255.99) as u8,
            g: (self.g()*255.99) as u8,
            b: (self.b()*255.99) as u8,
        }
    }
}

