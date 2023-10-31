#![allow(dead_code)]
use crate::glyphdetect::picture::*;
use crate::glyphdetect::point::*;
use crate::glyphdetect::color::*;
use crate::glyphdetect::fsm::*;
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
    pub pixels: Vec<(Point, Color)>,
    pub b: BoundingBox,
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

pub const SURROUNDING: [Point; 8] = [
    Point{x: 1, y: 0},
    Point{x:0, y: -1},
    Point{x: -1, y: 0},
    Point{x: 0, y: 1},
    Point{x: 1, y: 1},
    Point{x: 1, y: -1},
    Point{x: -1, y: -1},
    Point{x: -1, y: 1},
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

    for (index, pixel) in pic.pixels.iter().enumerate() {
        // This is if the pixel is in our we_care color list
        let pos = get_pos(index as i32, pic.width);
        let mut is_pixel_important = false;

        for important_color in we_care.iter() {
            if important_color == pixel {
                is_pixel_important = true;
            }
        }
        // If the pixel isn't important we do NOT CARE, but if there is no way it could be
        // important, then we have to give it a chance :3
        if !is_pixel_important && we_care.len() != 0 {
            continue;
        }
    
        if pixel == &WHITE {
            continue;
        }

        // If a pixel hasn't already been visited by the flood fill algorithm we will fill it
        // But if it has it would only lead to duplications
        if !visited[index] {
            let mut glyph = Glyph {color: *pixel, pixels: vec![], b: BoundingBox{u_left: Point::zero(), l_right: Point::zero()}};           
            glyph = flood_fill(&pic, glyph, &mut visited, pos);
            let mut max = Point{x: 0, y: 0};
            let mut min = Point{x: i32::MAX, y: i32::MAX};
            // Get the bounding box of the glyph (useful for other stuff)
            for pixel in glyph.pixels.iter() {
                let ppos = pixel.0;
                if ppos.x >= max.x {
                    max.x = pos.x;
                }
                if ppos.y >= max.y {
                    max.y = pos.y;
                }
                if ppos.x <= min.x {
                    min.x = pos.x;
                }
                if ppos.y <= min.y {
                    min.y = pos.y
                }
            }
            glyph.b.u_left = min;
            glyph.b.l_right = max;
            // Puts all of the pixels into the frame of the glyph coordinates, useful for
            // comparisons, if we need positional we just use the bounding box, and if we ever for
            // some reason need the real position back we can just add the bounding box back
            glyph_list.push(glyph);
        }
    }
    return glyph_list;
}

/// Returns a group of connected pixels (glyph) with the same color that are touching in any way
/// Starts off with a seed pixel and then moves on from there 
/// Visited is a list of bools the same size as the pic, stating if that pixel had already been
/// visited by this function
fn flood_fill(pic: &Picture, mut glyph: Glyph, visited: &mut Vec<bool>, pixel: Point) -> Glyph {
    // List of pixels we need to address
    let mut queue: Vec<Point> = vec![];
    // The current pixel we are looking around
    let mut selected_pixel = pixel;
    let selected_color = pic.pixels[get_index(selected_pixel, pic.width)];
    loop {
        // if it's not visited already we will visit it
        if visited[get_index(selected_pixel, pic.width)] == false {
            // Set the visited point to true, and add the pixel to the glyph
            visited[get_index(selected_pixel, pic.width)] = true;
            glyph.pixels.push((selected_pixel, selected_color));
        }
        
        // Gather the SURROUNDING pixels
        for i in 0..8 {
            let new_point = selected_pixel + SURROUNDING[i];
            // Guard statements about going out of bounds
            if new_point.x < 0 || new_point.x >= pic.width as i32 {
                continue;
            }
            if new_point.y < 0 || new_point.y >= pic.height as i32 {
                continue;
            }
            // If the surrounding pixel has already been visited we don't care about it
            if visited[get_index(new_point, pic.width)] == true {
                continue;
            }
            let new_pixel = pic.pixels[get_index(new_point, pic.width)];
            if new_pixel != selected_color {
                continue;
            }
            queue.push(new_point);
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



