// Usage: cargo run --release -- filename.png
// (requires sdl2-dev packages)
//
// Press Q to perform some very basic line detection,
// A,D moves selection
// H,L moves selected column
// The resulting images will be saved to ./img/
extern crate image;
extern crate sdl2;
use image::GenericImage;
use image::ImageBuffer;
use image::Rgb;
use image::Pixel;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::env;

pub fn run(png: &Path) {
    let img = image::open(png).unwrap();
    let (w, h) = img.dimensions();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", w, 600)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut path: String = png.to_string_lossy().to_string();
    let mut i = 0;
    let mut selected = 0;
    let mut ysort = 0;
    let mut show_selected = true;
    let mut speed = 1u32;
    let mut move_faster = false;

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                Event::KeyDown {keycode: Option::Some(keycode), ..} => match keycode {
                    Keycode::Q => {
                        let path_new = format!("img/tmp_{:08}.png", i);
                        unscramble(&path, &path_new, 204.0, ysort);
                        path = path_new;
                        ysort += 10;
                    }
                    Keycode::R => {
                        let path_new = format!("img/tmp_{:08}.png", i);
                        rotate(&path, &path_new);
                        path = path_new;
                    }
                    Keycode::A => selected -= if move_faster { speed } else { 1 },
                    Keycode::D => selected += if move_faster { speed } else { 1 },
                    Keycode::W => speed += 1,
                    Keycode::S => speed -= 1,
                    Keycode::H => {
                        let path_new = format!("img/tmp_{:08}.png", i);
                        let s = if move_faster { speed } else { 1 };
                        move_col(&path, &path_new, selected, -(s as i32));
                        selected -= s;
                        path = path_new;
                    }
                    Keycode::L => {
                        let path_new = format!("img/tmp_{:08}.png", i);
                        let s = if move_faster { speed } else { 1 };
                        move_col(&path, &path_new, selected, s as i32);
                        selected += s;
                        path = path_new;
                    }
                    Keycode::Space => {
                        show_selected = !show_selected;
                        println!("Selected: {}, speed: {}", selected, speed);
                    }
                    Keycode::F => move_faster = !move_faster,
                    _ => {}
                }
                _ => {}
            }
        }
        let texture = texture_creator.load_texture(&path).unwrap();
        canvas.copy(&texture, None, None).expect("Render failed");
        if show_selected {
            canvas.draw_line((selected as i32, 0), (selected as i32, 2000));
        }
        canvas.present();
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    run(Path::new(filename));
}

fn rotate(path: &str, save_to: &str) {
    let img = image::open(path).unwrap();

    let mut npx = vec![];
    let (w, h) = img.dimensions();
    for x in 0..w {
        let mut px = vec![];
        for y in 0..h {
            let p = img.get_pixel(x, y);
            let p = p.to_rgb();
            px.push(p);
        }
        npx.push(((x + 10)%w, px));
    }

    npx.sort_by(|a, b| a.0.cmp(&b.0));
    let npx = npx.into_iter().map(|a| Vline::new(a.1)).collect();

    Vline::to_image(npx, save_to);
}

fn move_col(path: &str, save_to: &str, mut s: u32, mut l: i32) {
    let mut img = image::open(path).unwrap();

    let l1 = if l > 0 { 1 } else if l < 0 { -1 } else { 0 };
    while l != 0 {
        let (w, h) = img.dimensions();
        let x = s % w;
        let x2 = (x as i32 + l1) as u32;
        for y in 0..h {
            let p1 = img.get_pixel(x, y);
            let p2 = img.get_pixel(x2, y);
            img.put_pixel(x, y, p2);
            img.put_pixel(x2, y, p1);
        }
        s += l1 as u32;
        l -= l1;
    }

    img.save(save_to);
}

fn unscramble(path: &str, save_to: &str, thres: f64, ymin: u32) {
    let img = image::open(path).unwrap();

    let (w, h) = img.dimensions();
    let mut vl = vec![];
    let black = Rgb { data: [0, 0, 0] };
    for x in 0..w {
        let mut is_black = true;
        let mut is_white = false;
        let mut blackness = 0;
        let mut px = vec![];
        for y in 0..h {
            let p = img.get_pixel(x, y);
            let p = p.to_rgb();
            //if y >= 30 && y <= 65 {
            //369, 412
            if y >= ymin && y <= ymin + 80 {
            //if y >= 415 && y <= 500 {
                //let greenish = Rgb { data: [0xa1, 0xb9, 0xaa] }; //a1b9aa
                //let thres = 200.0;
                if is_black && pixel_diff(&p, &black) > thres {
                    is_black = false;
                    if blackness == 0 {
                        is_white = true;
                    }
                } else if is_white {
                    blackness -= (pixel_diff(&p, &black) > thres) as i32;
                } else {
                    blackness += is_black as i32;
                }
            }
            px.push(p);
        }
        vl.push((Vline::new(px), blackness));
    }

    vl.sort_by(|a, b| b.1.cmp(&a.1));
    let vl = vl.into_iter().map(|a| a.0).collect();
    
    Vline::to_image(vl, save_to);
}

#[derive(Clone, Debug)]
struct Vline {
    pixels: Vec<Rgb<u8>>
}

impl Vline {
    fn new(pixels: Vec<Rgb<u8>>) -> Vline {
        Vline { pixels }
    }
    fn to_image(v: Vec<Vline>, path: &str) {
        let (w, h) = (v.len(), v[0].pixels.len());
        let mut img = ImageBuffer::new(w as u32, h as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = v[x as usize].pixels[y as usize];
        }
        
        img.save(path);
    }
}

fn pixel_diff(a: &Rgb<u8>, b: &Rgb<u8>) -> f64 {
    let c = a.channels();
    let d = b.channels();
    let rmean = (c[0] as i32 + d[0] as i32)/2;
    let r = c[0] - d[0];
    let g = c[1] - d[1];
    let b = c[2] - d[2];
    let (r, g, b) = (r as i32, g as i32, b as i32);

    f64::sqrt(((((512 + rmean) * r * r)>>8) + 4*g*g + (((767-rmean)*b*b)>>8)) as f64)
}
/*
// https://stackoverflow.com/questions/5392061/algorithm-to-check-similarity-of-colors
double ColourDistance(RGB e1, RGB e2)
{
  long rmean = ( (long)e1.r + (long)e2.r ) / 2;
  long r = (long)e1.r - (long)e2.r;
  long g = (long)e1.g - (long)e2.g;
  long b = (long)e1.b - (long)e2.b;
  return sqrt((((512+rmean)*r*r)>>8) + 4*g*g + (((767-rmean)*b*b)>>8));
}
*/
