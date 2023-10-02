use image::io::Reader;
use core::ops::{Add, Sub}; 

// To get all my functions:
// Find all my dirs
// Parse the function for its symbol and its inputs
// By this point we have all of the symbols for all of the functions
// Now we find all the dir signs, find al the functions associated with them
// the inputs will all be behind the dir until the picture and the outputs in front of it until the green
// If all of the colors are not present in the function, error
// find the start function this way by finding the dir with green only in front of it
// build the sequence of functions for it in an abstract syntax tree
// outputs have to have green on them
//
//// fill till you hit an un

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point{x: i32, y: i32}

impl Point {
    fn zero() -> Self {
        return Point{x: 0, y: 0}
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.y,
            y: self.y - other.y,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

const RED: Color = Color{r: 255, g: 0, b: 0};
const GREEN: Color = Color{r: 0, g: 255, b: 0};
const BLUE: Color = Color{r: 0, g: 0, b: 255};
const YELLOW: Color = Color{r: 255, g: 255, b: 0};
const WHITE: Color = Color{r: 255, g: 255, b: 255};

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
    b: BoundingBox,
}

impl Glyph {
    fn zero() -> Self {
        Glyph{
            color: Color{r: 0, g: 0, b: 0},
            pixels: vec![],
            b: BoundingBox { u_left: Point::zero(), l_right: Point::zero()}
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct BoundingBox {
    u_left: Point,
    l_right: Point,
}
impl Sub for BoundingBox {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            u_left: self.u_left - other.u_left,
            l_right: self.l_right - other.l_right,
        }
    }
}



#[derive(Debug)]
pub struct IdentifiedGlyph{
    glyph: Glyph,
    pub identifier: Glyphies,
    input: Vec<Color>,
    output: Vec<Color>,
}

#[derive(Debug, Clone)]
pub struct Definition{
    glyph: Glyph,
    pub identifier: Glyphies,
    inputs: Vec<(BoundingBox, Color)>,
    outputs: Vec<(BoundingBox, Color)>,
    id: usize, // It's position in the definitions vector
}

#[derive(Debug, Clone)]
pub struct Picture {
    pixels: Vec<Pixel>,
    width: i32,
    height: i32,
}

enum Axis {
    X,
    Y,
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


const BUILTIN_FILE_NAMES: [&str; 1] = [
    "symbols/add.png",
    // "symbols/mul.png",
    // "symbols/sub.png",
    // "symbols/div.png",
];

/// Loads in the 'dir' symbol, needs special attention because it loads in every other symbol
/// Returns a list because it's meant to be built off of
pub fn load_dir() -> Vec<Definition> {
    let path = "symbols/dir.png";
    let p = open_pic(path);
    let inc = Color{r: 127, g: 201, b: 255};
    let outc = Color{r: 127, g: 255, b: 142};
    let c = vec![BLUE, inc, outc];
    println!("glyphs in load_dir");
    let g = gather_glyphs(p, c);
    println!("{:?}", g);
    let mut dout: Glyph = Glyph::zero();
    let mut din: Glyph = Glyph::zero();
    let mut dg: Glyph = Glyph::zero();
    for glyph in g {
        if glyph.color == BLUE {
            dg = glyph;
        }
        else if glyph.color == inc {
            din = glyph;
        }
        else if glyph.color == outc {
            dout = glyph;
        }
    }
    let neg1 = Point{x: 1, y: 1};
    let minus = BoundingBox {u_left: dg.b.u_left + neg1, l_right: dg.b.u_left+ neg1};
    let d = Definition{
        glyph: dg.clone(),
        identifier: Glyphies::Dir,
        inputs: vec![(din.b - minus, din.color)],
        outputs: vec![(dout.b - minus, dout.color)],
        id: 0,
    };
    println!("loading dir");
    println!("{:?}", d);
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    return vec![d];
}

pub fn load_builtin_definitions() -> Vec<Definition> {
    let mut definitions = load_dir();
    let dir = definitions[0].clone();
    for path in BUILTIN_FILE_NAMES {
        let p = open_pic(path);
        let mut input_colors: Vec<Color> = vec![];
        let mut output_colors: Vec<Color> = vec![];
        let glyphs = gather_glyphs(p.clone(), vec![]);

        let mut path_dir = Glyph::zero();
        let mut yellow_box = Glyph::zero();
        let mut unidentified_builtin = Glyph::zero();
        // Find Dir
        for g in glyphs.iter() {
            if g.color == YELLOW {
                yellow_box = g.clone();
                continue;
            }
            if g.pixels.len() != dir.glyph.pixels.len() {
                continue;
            }
            if g.color != dir.glyph.color {
                continue;
            }
            // Checks all of the pixels are the same
            if !dir.glyph.pixels.iter().all(|item| g.pixels.contains(item)) {
                unidentified_builtin = g.clone();
                continue
            }
            path_dir = g.clone(); // By this point it must be dir
        }
        // Find all the input colors
        println!("{:?}", path_dir);
        println!("{:?}", dir.inputs[0].0);
        let mut i_in = path_dir.b.u_left + dir.inputs[0].0.u_left;
        println!("{:?}", i_in);
        loop {
            if i_in.x <= 0 {
                break;
            }
            let looking = p.pixels[get_index(i_in, p.width)];
            println!("{:?}", looking);
            // When we hit the picture break
            if looking.color == YELLOW {
                break;
            }
            if looking.color == WHITE {
                break;
            }
            if looking.color == BLUE {
                i_in.x -= 1;
                continue;
            }
            input_colors.push(looking.color);
            i_in.x -= 1;
        }
        // Find all the output Colors
        let mut i_out = path_dir.b.u_left + dir.outputs[0].0.u_left;
        loop {
            if i_out.x >= p.width {
                break;
            }
            let looking = p.pixels[get_index(i_out, p.width)];
            // When we hit the flow break
            if looking.color == WHITE {
                break;
            }
            if looking.color == GREEN {
                break;
            }
            if looking.color == BLUE {
                i_in.x += 1;
                continue;
            }
            output_colors.push(looking.color);
            i_out.x += 1;
        }
        // All inputs and outputs will be inside the function
        let function_bounds = yellow_box.b;
        // Find the positions of the input colors
        let identifier = match path {
            "symbols/add.png" => Glyphies::Add,
            "symbols/mul.png" => Glyphies::Mul,
            "symbols/sub.png" => Glyphies::Sub,
            "symbols/div.png" => Glyphies::Div,
            _ => Glyphies::ERRRORRORORR,
        };
        let mut builtin = Definition{glyph: unidentified_builtin, identifier, id: definitions.len(), inputs: vec![], outputs: vec![]};
        builtin.glyph.color = BLUE;
        for g in glyphs.iter() {
            if !input_colors.contains(&g.color) && !output_colors.contains(&g.color) {
                continue
            }
            if g.b.u_left.x < yellow_box.b.u_left.x || g.b.u_left.y < yellow_box.b.u_left.y {
                continue;
            }
            if g.b.l_right.x > yellow_box.b.l_right.x || g.b.l_right.y > yellow_box.b.l_right.y {
                continue;
            }
            if input_colors.contains(&g.color) {
                builtin.inputs.push((g.b, g.color));
                continue;
            }
            if output_colors.contains(&g.color) {
                builtin.outputs.push((g.b, g.color));
                continue;
            }
        }
        definitions.push(builtin);

    }
    return definitions;

}

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
            // identified_glyphs.push(IdentifiedGlyph{glyph: to_ident.clone(), identifier: ident.identifier.clone()});
        }
    }
    return identified_glyphs
}

/// Gathers every group of connected pixels with the same colors
/// Only gathers groups of colors we care about
pub fn gather_glyphs(image: Picture, we_care: Vec<Color>) -> Vec<Glyph> {
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
        }
        // If the pixel isn't important we do NOT CARE
        if !is_pixel_important && we_care.len() != 0 {
            continue;
        }

        if !visited[get_index(pixel.point, image.width)] {
            let mut glyph = Glyph {color: pixel.color, pixels: vec![], b: BoundingBox{u_left: Point::zero(), l_right: Point::zero()}};           
            glyph = flood_fill(&image, glyph, &mut visited, pixel.clone());
            let mut max = Point{x: 0, y: 0};
            let mut min = Point{x: i32::MAX, y: i32::MAX};
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
            glyph.b.u_left = min;
            glyph.b.l_right = max;
            for pixel in glyph.pixels.iter_mut() {
                pixel.point = Point{x: pixel.point.x - min.x, y: pixel.point.y - min.y};
            }
            glyph_list.push(glyph);
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

fn flood_fill(image: &Picture, mut glyph: Glyph, visited: &mut Vec<bool>, pixel: Pixel) -> Glyph {
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
            let new_point = Point{x: new_x as i32, y: new_y as i32};

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
pub fn open_pic(path: &str) -> Picture {
    let img = Reader::open(path).unwrap().decode().unwrap().to_rgb8();

    let width = img.width();
    let height = img.height();
    let data: Vec<u8>= img.into_raw();

    let mut ret_img: Picture = Picture{
        pixels: vec![],
        width: width as i32,
        height: height as i32,
    };

    for c in 0..(height * width) {
        let c3 = (c * 3) as usize;
        let point = get_pos(c as i32, width as i32);
        let color = Color { r: data[c3], g: data[c3 + 1], b: data[c3 + 2]}; 
        ret_img.pixels.push(Pixel{color, point});
    }
    return ret_img;
}

/// Starts from the top left and goes down to the bottom right
/// Returns (x,y)
pub fn get_pos(index: i32, width: i32) -> Point {
    let x = index % width; 
    let y = index / width;
    return Point{x: x as i32, y: y as i32};
}

/// Opposite of get_pos
/// Takes in an x and y and returns the corresponding index
pub fn get_index(point: Point, width: i32) -> usize {
    return (point.y * width + point.x) as usize;
}

