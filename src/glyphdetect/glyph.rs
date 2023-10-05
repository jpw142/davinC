use crate::glyphdetect::picture::*;
use crate::glyphdetect::point::*;
use crate::glyphdetect::color::*;
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



/// A group of connected pixels to be identified
/// Bounding box is the upper left and the lowest right
#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Identified{
    glyph: Glyph,
    pub identifier: Glyphies,
    input: Vec<Color>,
    output: Vec<Color>,
    // flowin: i32
    // flowout: i32
    // These will be the id's in the identified vector of the flow
}

/// A definition of a function
#[derive(Debug, Clone)]
pub struct Definition{
    glyph: Glyph,
    pub identifier: Glyphies,
    inputs: Vec<(BoundingBox, Color)>,
    outputs: Vec<(BoundingBox, Color)>,
    id: usize, // It's position in the definitions vector
}


enum Bounds {
    xinf,
    yinf,
    x(i32),
    y(i32),

}
enum Axis {
    X,
    Y,
}

/// Identifiers for definition glyphs, like a function name
#[derive(Debug, Copy, Clone)]
pub enum Glyphies {
    Dir,
    Add,
    Mul,
    Div,
    Sub,
    Assign,
    CustomFunction,
    ERRRORRORORR,
}


const BUILTIN_FILE_NAMES: [&str; 4] = [
    "symbols/add.png",
    "symbols/mul.png",
    "symbols/sub.png",
    "symbols/div.png",
];


/// Loads in the 'dir' symbol, needs special attention because it loads in every other symbol
/// Returns a list of definitions because it's meant to be built off of
fn load_dir() -> Vec<Definition> {
    let path = "symbols/dir.png";
    let p = open_pic(path);
    let inc = Color{r: 127, g: 201, b: 255};
    let outc = Color{r: 127, g: 255, b: 142};
    let c = vec![BLUE, inc, outc];
    let g = gather_glyphs(&p, c);
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
    let minus = BoundingBox {u_left: dg.b.u_left, l_right: dg.b.u_left};
    let d = Definition{
        glyph: dg.clone(),
        identifier: Glyphies::Dir,
        inputs: vec![(din.b - minus, din.color)],
        outputs: vec![(dout.b - minus, dout.color)],
        id: 0,
    };
    return vec![d];
}

/// Loads in the builtin definitions as they will never be included in the actual program so we
/// have to do it special
/// Also it is very hard to define their behavior so it's easier to do it once
/// Returns a vector of definitions 
pub fn load_builtin_definitions() -> Vec<Definition> {
    let mut definitions = load_dir();
    let dir = definitions[0].clone();
    for path in BUILTIN_FILE_NAMES {
        let p = open_pic(path);
        let mut input_colors: Vec<Color> = vec![];
        let mut output_colors: Vec<Color> = vec![];
        let glyphs = gather_glyphs(&p, vec![]);
        // The dir that is in the picture
        let mut path_dir = Glyph::zero();
        // The yellow picture definition box
        let mut yellow_box = Glyph::zero();
        // The unidentified function that we are trying to define
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
                continue
            }
            path_dir = g.clone(); // By this point it must be dir
        }

        // Find glyph
        for g in glyphs.iter() {
            if g.color == YELLOW {
                continue;
            }
            // Assuming here that it's not the exact same length as dir
            if g.pixels.len() == dir.glyph.pixels.len() {
                continue;
            }
            if g.color != BLUE {
                continue;
            }
            if dir.glyph.pixels.iter().all(|item| g.pixels.contains(item)) {
                continue
            }
            unidentified_builtin = g.clone() // By this point it must be our builtin
        }

        // Find all the input colors (Left of dir)
        // This is where the input would be relative to the picture coordinates
        let mut i_in = path_dir.b.u_left + dir.inputs[0].0.u_left; 
                                                                   
        loop {
            if i_in.x <= 0 {
                break;
            }
            let looking = p.pixels[get_index(i_in, p.width)];
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

        // Find all the output Colors (Right of dir)
        // This is where the output would be relative to the picture coordinates
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
        // what we have to minus to put the inputs and outputs in the frame of builtin coords
        let minus = BoundingBox{u_left: builtin.glyph.b.u_left, l_right: builtin.glyph.b.u_left};
        for g in glyphs.iter() {
            if !input_colors.contains(&g.color) && !output_colors.contains(&g.color) {
                continue
            }
            // Makes sure these inputs are actually in the image
            if g.b.u_left.x < yellow_box.b.u_left.x || g.b.u_left.y < yellow_box.b.u_left.y {
                continue;
            }
            if g.b.l_right.x > yellow_box.b.l_right.x || g.b.l_right.y > yellow_box.b.l_right.y {
                continue;
            }
            if input_colors.contains(&g.color) {
                builtin.inputs.push((g.b - minus, g.color));
                continue;
            }
            if output_colors.contains(&g.color) {
                builtin.outputs.push((g.b - minus, g.color));
                continue;
            }
        }
        definitions.push(builtin);

    }
    return definitions;

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

/// Gathers every group of connected pixels with the same colors
/// Only gathers groups of colors we care about
/// If the vec of colors we care about is empty, collect every pixel that has a value other than
/// white
pub fn gather_glyphs(pic: &Picture, we_care: Vec<Color>) -> Vec<Glyph> {
    let mut glyph_list = vec![];

    // Creates a list to see if we have visited each pixel
    // (For flood fill algorithm)
    let mut visited = vec![false; pic.pixels.len()];

    for pixel in pic.pixels.iter() {
        // This is if the pixel is in our we_care color list
        let mut is_pixel_important = false;

        for important_color in we_care.iter() {
            if important_color == &pixel.color {
                is_pixel_important = true;
            }
        }
        // If the pixel isn't important we do NOT CARE, but if there is no way it could be
        // important, then we have to give it a chance :3
        if !is_pixel_important && we_care.len() != 0 {
            continue;
        }
    
        if pixel.color == WHITE {
            continue;
        }

        // If a pixel hasn't already been visited by the flood fill algorithm we will fill it
        // But if it has it would only lead to duplications
        if !visited[get_index(pixel.point, pic.width)] {
            let mut glyph = Glyph {color: pixel.color, pixels: vec![], b: BoundingBox{u_left: Point::zero(), l_right: Point::zero()}};           
            glyph = flood_fill(&pic, glyph, &mut visited, pixel.clone());
            let mut max = Point{x: 0, y: 0};
            let mut min = Point{x: i32::MAX, y: i32::MAX};
            // Get the bounding box of the glyph (useful for other stuff)
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
            // Puts all of the pixels into the frame of the glyph coordinates, useful for
            // comparisons, if we need positional we just use the bounding box, and if we ever for
            // some reason need the real position back we can just add the bounding box back
            for pixel in glyph.pixels.iter_mut() {
                pixel.point = Point{x: pixel.point.x - min.x, y: pixel.point.y - min.y};
            }
            glyph_list.push(glyph);
        }
    }
    return glyph_list;
}

/// Returns a group of connected pixels (glyph) with the same color that are touching in any way
/// Starts off with a seed pixel and then moves on from there 
/// Visited is a list of bools the same size as the pic, stating if that pixel had already been
/// visited by this function
fn flood_fill(pic: &Picture, mut glyph: Glyph, visited: &mut Vec<bool>, pixel: Pixel) -> Glyph {
    // List of pixels we need to address
    let mut queue: Vec<Pixel> = vec![];
    // The current pixel we are looking around
    let mut selected_pixel = pixel;
    loop {
        // if it's not visited already we will visit it
        if visited[get_index(selected_pixel.point, pic.width)] == false {
            // Set the visited point to true, and add the pixel to the glyph
            visited[get_index(selected_pixel.point, pic.width)] = true;
            glyph.pixels.push(selected_pixel);
        }
        
        // Gather the SURROUNDING pixels
        for i in 0..8 {
            let (off_x, off_y) = SURROUNDING[i];
            let new_x = selected_pixel.point.x as i32 + off_x;
            let new_y = selected_pixel.point.y as i32 + off_y;
            // Guard statements about going out of bounds
            if new_x < 0 || new_x >= pic.width as i32 {
                continue;
            }
            if new_y < 0 || new_y >= pic.height as i32 {
                continue;
            }
            // The point of the pixel surrounding it
            let new_point = Point{x: new_x as i32, y: new_y as i32};
            // If the surrounding pixel has already been visited we don't care about it
            if visited[get_index(new_point, pic.width)] == true {
                continue;
            }
            let new_pixel = pic.pixels[get_index(new_point, pic.width)];
            if new_pixel.color != pixel.color {
                continue;
            }
            queue.push(new_pixel);
        }
        // If we have no more pixels in the queue to look at we are done
        let opt_pixel = queue.pop();
        if opt_pixel.is_none() {
            return glyph;
        }
        else {
            selected_pixel = opt_pixel.unwrap();
        }
    }
}


/// Creates the definition when the dir is identified and the picture definition is found
/// You could identify it in bounding boxes to solve the infinitley expanding principle
/// Link the bounding boxes together with their surrounding bounding boxes
pub fn create_definition(subpicture: &Picture, inputs: Vec<Color>, outputs: Vec<Color>) -> Definition {
    let glyphs = gather_glyphs(subpicture, vec![]); 
    // Find the function color
    for g in glyphs.iter() {
    
    }
    // Find the function glyph
    // Find all the inputs and outputs
    todo!()
}

/// Checks to see if a glyph fits the definition
/// Only checks against the pixels of the definition, not inputs or such
pub fn fits_definition(definition: &Definition, glyph: &Glyph) -> bool {
    // Checks the glyph fitsx
    // Checks the inputs fit
    // Checks the outputs fit
    // Checks that all the infinitley expandable things are equal where they need to be
    todo!();
}


pub enum IdentifyError {
    MissingFlowIn,
    MissingOutputFlow,
    MissingInput,
    MissingOutput,
}

pub fn identify(pic: &Picture, definition: &Definition, glyph: &Glyph) -> Result<Identified, IdentifyError> {
    todo!()
}
