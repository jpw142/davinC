use image::io::Reader;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point{x: usize, y: usize}

impl Point {
    fn zero() -> Self {
        return Point{x: 0, y: 0}
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pixel {
    color: Color,
    point: Point,
}
#[derive(Debug, Clone)]
/// A group of connected pixels to be identified
/// Bounding box is the upper left and the lowest right
pub struct Glyph {
    pub color: Color,
    pub pixels: Vec<Pixel>,
    u_left: Point,
    l_right: Point,
}

#[derive(Debug)]
pub struct IdentifiedGlyph{
    glyph: Glyph,
    pub identifier: Glyphies,
}

#[derive(Debug, Clone)]
pub struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Glyphies {
    Dir,
    Add,
    Mul,
    Div,
    Sub,
    Assign,
    ERRRORRORORR,
}


const BUILTIN_FILE_NAMES: [&str; 5] = [
    "symbols/dir.png",
    "symbols/add.png",
    "symbols/mul.png",
    "symbols/sub.png",
    "symbols/div.png",
];


pub fn identify_glyphs(identified: Vec<IdentifiedGlyph>, to_identify: Vec<Glyph>) -> Vec<IdentifiedGlyph> {
    let mut identified_glyphs = vec![];
    for ident in identified.iter() {
        for to_ident in to_identify.iter() {
            if ident.glyph.color != to_ident.color {
                continue;
            }
            if ident.glyph.pixels.len() != to_ident.pixels.len() {
                continue;
            }
            if !ident.glyph.pixels.iter().all(|item| to_ident.pixels.contains(item)) {
                continue
            }
            identified_glyphs.push(IdentifiedGlyph{glyph: to_ident.clone(), identifier: ident.identifier.clone()});
        }
    }
    return identified_glyphs
}

pub fn load_builtin_glyphs() -> Vec<IdentifiedGlyph> {
    let mut vec_glyphs = vec![];
    for path in BUILTIN_FILE_NAMES {
        let image = open_image(path);
        let glyphs = gather_glyphs(image, vec![Color{r: 0, g: 0, b: 255}]);
        assert!(glyphs.len() == 1);
        let identifier = match path {
            "symbols/dir.png" => Glyphies::Dir,
            "symbols/add.png" => Glyphies::Add,
            "symbols/mul.png" => Glyphies::Mul,
            "symbols/div.png" => Glyphies::Div,
            "symbols/sub.png" => Glyphies::Sub,
            _ => Glyphies::ERRRORRORORR,
        };
        vec_glyphs.push(IdentifiedGlyph{glyph: glyphs[0].clone(), identifier})
    }
    return vec_glyphs;
}

/// Gathers every group of connected pixels with the same colors
/// Only gathers groups of colors we care about
pub fn gather_glyphs(image: Image, we_care: Vec<Color>) -> Vec<Glyph> {
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
                let mut max = Point{x: 0, y: 0};
                let mut min = Point{x: usize::MAX, y: usize::MAX};
                for pixel in glyph.pixels.iter() {
                    if pixel.point.x >= max.x {
                        max.x = pixel.point.x;
                    }
                    if pixel.point.y >= max.y {
                        max.y = pixel.point.y;
                    }
                    if pixel.point.x <= min.x {
                        min.x = pixel.point.x;
                    }
                    if pixel.point.y <= min.y {
                        min.y = pixel.point.y
                    }
                }
                glyph.u_left = min;
                glyph.l_right = max;
                for pixel in glyph.pixels.iter_mut() {
                    pixel.point = Point{x: pixel.point.x - min.x, y: pixel.point.y - min.y};
                }
                glyph_list.push(glyph);
            }
        }
    }
    return glyph_list;
}

const SURROUNDING: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn flood_fill(image: &Image, mut glyph: Glyph, visited: &mut Vec<bool>, pixel: Pixel) -> Glyph {
    // List of pixels we need to address
    let mut queue: Vec<Pixel> = vec![];
    // The current pixel we are looking around
    let mut selected_pixel = pixel;
    loop {
        // if it's 
        if visited[get_index(selected_pixel.point, image.width)] == false {
            // Set the visited point to true, and add the pixel to the glyph
            visited[get_index(selected_pixel.point, image.width)] = true;
            glyph.pixels.push(selected_pixel);
        }
        
        // Gather the SURROUNDING pixels
        for i in 0..8 {
            let (off_x, off_y) = SURROUNDING[i];
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
pub fn open_image(path: &str) -> Image {
    let img = Reader::open(path).unwrap().decode().unwrap().to_rgb8();

    let width = img.width();
    let height = img.height();
    let data: Vec<u8>= img.into_raw();

    let mut ret_img: Image = Image{
        pixels: vec![],
        width: width as usize,
        height: height as usize,
    };

    for c in 0..(height * width) {
        let c3 = (c * 3) as usize;
        let point = get_pos(c as usize, width as usize);
        let color = Color { r: data[c3], g: data[c3 + 1], b: data[c3 + 2]}; 
        ret_img.pixels.push(Pixel{color, point});
    }
    return ret_img;
}

/// Starts from the top left and goes down to the bottom right
/// Returns (x,y)
pub fn get_pos(index: usize, width: usize) -> Point {
    let x = index % width; 
    let y = index / width;
    return Point{x, y};
}

/// Opposite of get_pos
/// Takes in an x and y and returns the corresponding index
pub fn get_index(point: Point, width: usize) -> usize {
    return point.y * width + point.x;
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
