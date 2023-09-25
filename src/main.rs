use image::io::Reader;
use image::{DynamicImage, ImageError, Rgb};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point{x: usize, y: usize}

impl Point {
    fn zero() -> Self {
        return Point{x: 0, y: 0}
    }
}


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

#[derive(Debug, Clone)]
struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}


/// Gathers every group of connected pixels with the same colors
/// Only gathers groups of colors we care about
fn gather_glyphs(image: Image, we_care: Vec<Color>) -> Vec<Glyph> {
    let mut glyph_list = vec![];

    // Creates a list to see if we have visited each pixel
    // (Foor flood fill algorithm)
    let mut visited = vec![false; image.pixels.len()];

    for pixel in image.pixels.iter() {
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

            if !visited[get_index(pixel.point, image.width)] {
                let mut glyph = Glyph {color: pixel.color, pixels: vec![], u_left: Point::zero(), l_right: Point::zero()};           
                glyph = flood_fill(&image, glyph, &mut visited, pixel.clone());

                glyph_list.push(glyph);
            }

        }
    
    }
    return glyph_list;
}


fn flood_fill(image: &Image, mut glyph: Glyph, visited: &mut Vec<bool>, pixel: Pixel) -> Glyph {
    let surrounding: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    // List of pixels we need to address
    let mut queue: Vec<Pixel> = vec![];
    // The current pixel we are looking around
    let mut selected_pixel = pixel;
    loop {
        // Set the visited point to true, and add the pixel to the glyph
        visited[get_index(selected_pixel.point, image.width)] = true;
        glyph.pixels.push(selected_pixel);
        
        // Gather the surrounding pixels
        for i in 0..8 {
            let (off_x, off_y) = surrounding[i];
            let new_x = selected_pixel.point.x as i32 + off_x;
            let new_y = selected_pixel.point.y as i32 + off_y;
            // Guard statements about going out of bounds
            if new_x < 0 || new_x >= image.width as i32 {
                continue;
            }
            if new_y < 0 || new_y >= image.height as i32 {
                continue;
            }
            let new_point = Point{x: new_x as usize, y: new_y as usize};

            if visited[get_index(new_point, image.width)] == true {
                continue;
            }
            let new_pixel = image.pixels[get_index(new_point, image.width)];
            if new_pixel.color != pixel.color {
                continue;
            }
            queue.push(new_pixel);
        }
        let opt_pixel = queue.pop();
        if opt_pixel.is_none() {
            return glyph;
        }
        else {
            selected_pixel = opt_pixel.unwrap();
        }
    }
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
    let path = "blue.png";
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
        let point = get_pos(c as usize, width as usize);
        let color = Color { r: data[c3], g: data[c3 + 1], b: data[c3 + 2]}; 
        image.pixels.push(Pixel{color, point});
    }

    let mut used_colors: Vec<Color> = vec![Color{r: 255, b: 0, g: 0}, Color{r: 0, b: 255, g: 0}, Color{r: 0, b: 0, g: 255}];

    println!("{:?}", gather_glyphs(image, used_colors));
}
#[cfg(test)]
mod test {
    use crate::{Point, get_index, get_pos};

    #[test]
    fn test() {
        let point = Point{x: 10, y: 10};
        let width = 20;
        println!("{:?}", get_index(point, width));
        println!("{:?}", get_pos(get_index(point, width), width));
        assert!(get_pos(get_index(point, width), width) == point);
    }
}
