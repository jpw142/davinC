use image::io::Reader;
use image::{DynamicImage, ImageError, Rgb};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point{x: usize, y: usize}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pixel {
    color: Color,
    point: Point,
}

#[derive(Debug)]
/// A group of connected pixels to be identified
/// Bounding box is the upper left -1 and the lowest right - 1
struct Glyph {
    color: Color,
    pixels: Vec<Pixel>,
    u_left: Point,
    l_right: Point,
}

#[derive(Debug)]
struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}


/// Gathers every group of connected pixels with the same colors
/// Only gathers groups of colors we care about
fn gather_glyphs(image: Image, we_care: Vec<Color>) -> Vec<Glyph> {
    // Creates a list to see if we have visited each pixel
    // (Foor flood fill algorithm)
    let mut visited = vec![false; image.pixels.len()];

    for pixel in image.pixels {
        // This is if the pixel is in our we_care color list
        let mut is_pixel_important = false;

        for important_color in we_care.iter() {
            if important_color == &pixel.color {
                is_pixel_important = true;
            }
            // If the pixel isn't important we do NOT CARE
            if !is_pixel_important {
                continue;
            }

            let connected_pixels: Vec<Pixel> = vec![];

        }
    
    }
    return vec![];
}

fn flood_fill(image: Image, mut glyph: Glyph, mut visited_list: Vec<bool>, color: Color, pos: Point) -> Glyph {
    let index = get_index(pos, image.width);
    if visited_list[index] == true {
        return glyph; 
    }
    if image.pixels[index].color != color {
        return glyph;
    }
    glyph.pixels.push(image.pixels[index]);
    visited_list[index] = true;
    flood_fill(image, glyph, visited_list, color, pos)
    return glyph;
}

/// Opens image and given the file path is correct, returns the image
fn open_image(path: &str) -> Result<DynamicImage, ImageError> {
    let img = Reader::open(path)?.decode();
    return img;
}

/// Starts from the top left and goes down to the bottom right
/// Returns (x,y)
fn get_pos(index: usize, width: usize) -> Point {
    let x = index % width; 
    let y = index / width;
    return Point{x, y};
}

/// Opposite of get_pos
/// Takes in an x and y and returns the corresponding index
fn get_index(point: Point, width: usize) -> usize {
    return point.y * width + point.x;
}

fn main() {
    let path = "blank.png";
    let main_img = open_image(path).unwrap().to_rgb8();

    let width = main_img.width();
    let height = main_img.height();
    let data: Vec<u8>= main_img.into_raw();

    let mut image: Image = Image{
        pixels: vec![],
        width: width as usize,
        height: height as usize,
    };

    for c in 0..(height * width) {
        let c3 = (c * 3) as usize;
        let point = get_pos(c3, width as usize);
        let color = Color { r: data[c3], g: data[c3 + 1], b: data[c3 + 2]}; 
        image.pixels.push(Pixel{color, point});
    }
    print!("{:?}", image);

    let mut used_colors: Vec<Color> = vec![Color{r: 255, b: 0, g: 0}, Color{r: 0, b: 255, g: 0}, Color{r: 0, b: 0, g: 255}];
}
