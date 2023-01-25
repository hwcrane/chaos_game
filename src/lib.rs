use color_space::Rgb;
use image::RgbImage;
use polygon::Polygon;
use vec2::Vec2;

mod polygon;
mod vec2;

const N: usize = 10;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

pub fn run() {
    let mut poly = Polygon::new(N);
    let mut img : Vec<Vec<[f64;3]>> = (0..HEIGHT).map(|_| (0..WIDTH).map(|_| [0., 0., 0.]).collect()).collect();

    let mut point = Vec2::new(0.4, -0.5);
    let mut rndm_idx: usize;
    let mut vertex: &Vec2;
    let mut colour: Rgb;

    for _ in 0..(HEIGHT as usize * WIDTH as usize * 1000) {
        // Pick a random vertex
        rndm_idx = poly.random_vertex();
        vertex = &poly.verticies[rndm_idx];
        colour = poly.colours[rndm_idx];

        point.x += (vertex.x - point.x) * 0.5;
        point.y += (vertex.y - point.y) * 0.5;

        add_point(&mut img, &point, &colour);
    }

    let mut png_img = RgbImage::new(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let col = img[x as usize][y as usize];
            png_img.put_pixel(x, y, image::Rgb([col[0] as u8, col[1] as u8, col[2] as u8]))
            
        }
    }
    png_img.save("out.png").unwrap();
}

fn add_point(img: &mut Vec<Vec<[f64;3]>>, point: &Vec2, colour: &Rgb) {
    let enlarged_point = point * (HEIGHT as f64 / 2.1);

    let x = ((WIDTH / 2) as f64 + enlarged_point.x) as u32;
    let y = ((HEIGHT / 2) as f64 + enlarged_point.y) as u32;
    let pix = &mut img[x as usize][y as usize];
    let colour_scale = 1000. * (10./3.);
    pix[0] = f64::min(pix[0] + (colour.r / colour_scale), 255.);
    pix[1] = f64::min(pix[1] + (colour.g / colour_scale), 255.);
    pix[2] = f64::min(pix[2] + (colour.b / colour_scale), 255.);
    // img.put_pixel(x, y, Rgb([255, 255, 255]))
}
