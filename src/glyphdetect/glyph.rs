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
    pub pixels: Vec<(Point, Color)>,
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


const SURROUNDING: [Point; 8] = [
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
    let mut selected_color = pic.pixels[get_index(selected_pixel, pic.width)];
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


#[derive(Debug, Clone)]
pub struct FSM {
    pub states: Vec<State>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub t: Vec<(usize, Transition)>
}

#[derive(Debug, Clone, Copy)]
pub enum Transition {
    MoveRelativeState(usize, Point),
    MoveRelativeStateConsume(usize, Point, ColorType), // Move relative to a point from that state then consume
    Epsilon, // Change states for free
    CaptureType(ColorType, u8),
    EndCapture, // Ends a capture group
}


/// Creates the definition when the dir is identified and the picture definition is found
/// You could identify it in bounding boxes to solve the infinitley expanding principle
/// Link the bounding boxes together with their surrounding bounding boxes
pub fn create_definition(mut func_box: Picture, inputs: Vec<Color>, outputs: Vec<Color>) -> Definition{
    // Get the function color
    // If all the corners are the same color make the function color the corner color else blue
    let tl = func_box.pixels[get_index(Point {x: 0, y: 0}, func_box.width)];
    let tr = func_box.pixels[get_index(Point {x: func_box.width - 1, y: 0}, func_box.width)];
    let dl = func_box.pixels[get_index(Point {x: 0, y: func_box.height - 1}, func_box.width)];
    let dr = func_box.pixels[get_index(Point {x: func_box.width -1, y: func_box.width - 1}, func_box.width)];
    let mut ledger = ColorLedger {
        inputs,
        outputs,
        function: BLUE,
    };
    if (tl == tr) && (tl == dl) && (tl == dr) {
        ledger.function = tl;
        // Set them to white so they don't interfere with gather glyphs
        func_box.pixels[get_index(Point {x: 0, y: 0}, func_box.width)] = WHITE;
        func_box.pixels[get_index(Point {x: func_box.width - 1, y: 0}, func_box.width)] = WHITE;
        func_box.pixels[get_index(Point {x: 0, y: func_box.height - 1}, func_box.width)] = WHITE;
        func_box.pixels[get_index(Point {x: func_box.width -1, y: func_box.width - 1}, func_box.width)] = WHITE;
    }

    // If it's not in inputs, outputs, black, or the function color we don't care about it
    for mut pixel in func_box.pixels.iter_mut() {
        if ledger.identify(&pixel).is_none() {
            pixel = &mut WHITE;
        }
    }

    let mut definition = Definition{identifier: GlyphType::CustomFunction(0), instructions: vec![]};

    for _ in 0..4 {
        // Find the top left pixel, as that is the start point  
        let mut min = Point{x: func_box.width, y: func_box.height};
        for (i, pixel) in func_box.pixels.iter().enumerate() {
            if *pixel == WHITE {
                continue;
            }
            let pos = get_pos(i as i32, func_box.width);
            if pos.x <= min.x && pos.y <= min.y {
                min = pos;
            }
        }

        let head_pos = min;

        let mut visited = vec![false; (func_box.width * func_box.width) as usize];
        visited[get_index(head_pos, func_box.width)] = true;
        let fsm = FSM{states: follow(&func_box, &ledger, head_pos, head_pos, &mut visited)};
        definition.instructions.push(fsm);
        func_box = rotate(func_box);
    }
    return definition;
}

fn follow (func_box: &Picture, ledger: &ColorLedger, head_pos: Point, last_head_pos: Point, visited: &mut Vec<bool>) -> Vec<State> {
    let mut surrounding_pixels = vec![];
    for pos in SURROUNDING {
        let new_pos = head_pos + pos;
        if new_pos.x.is_negative() || new_pos.y.is_negative() {
            continue
        }
        if new_pos.x >= func_box.width || new_pos.y >= func_box.height {
            continue
        }
        let temp_color = func_box.pixels[get_index(new_pos, func_box.width)];
        if ledger.identify(&temp_color).is_some() {
            surrounding_pixels.push(pos);
        }
    }
    let len_surround = surrounding_pixels.len();
    let last_type = ledger.identify(&func_box.pixels[get_index(last_head_pos, func_box.width)]).unwrap();
    let cur_type = ledger.identify(&func_box.pixels[get_index(head_pos, func_box.width)]).unwrap();
    // Base case: 
    // A color will always have at least 2 neighbors unless it's at the end or beggining
    // And at the beginning headpos = last_headpos
    if len_surround == 1 && head_pos != last_head_pos {
        match cur_type {
            // If the current color is a loop we need a bit of special stuff before
            // We need to initialize a capture, make the loop part, and end the capture
            // And then we need the empty state at the end for the next recursion or as end state
            ColorType::Loop(id) => {
                match last_type {
                    ColorType::Loop(_) => {return vec![State{t: vec![]}];}, // no need, will be handled by back one
                    _ => {return vec![
                        State{t: vec![(1, Transition::CaptureType(last_type, id))]},
                        State{t: vec![(2, Transition::MoveRelativeStateConsume(1, head_pos - last_head_pos, last_type)), (2, Transition::Epsilon)]},
                        State{t: vec![(1, Transition::Epsilon), (3, Transition::EndCapture)]},
                        State{t: vec![]},

                    ]}
                }
            }
            _ => {return vec![State{t: vec![]}]} 
        }
    }
    let mut states = vec![];
    match cur_type {
        // If the current color is a loop we need a bit of special stuff before
        // We need to initialize a capture, make the loop part, and end the capture
        // And then we need the empty state at the end for the next recursion or as end state
        ColorType::Loop(id) => {
            match last_type {
                ColorType::Loop(_) => {states.push(State{t: vec![]});}, // no need, will be handled by back one
                _ => {states.append(&mut vec![
                    State{t: vec![(1, Transition::CaptureType(last_type, id))]},
                    State{t: vec![(2, Transition::MoveRelativeStateConsume(1, head_pos - last_head_pos, last_type)), (2, Transition::Epsilon)]},
                    State{t: vec![(1, Transition::Epsilon), (3, Transition::EndCapture)]},
                    State{t: vec![]},
                    
                ])}
            }
        }
        _ => {states.push(State{t: vec![]})} 
    }
    for pos in surrounding_pixels {
        let new_pos = head_pos + pos;
        // If it's already been accounted for don't visit it again
        if visited [get_index(new_pos, func_box.width)] {
            continue;
        }

        // Since it hasn't been visited, then visit it
        visited[get_index(new_pos, func_box.width)] = true;
        let mut rest_states = follow(func_box, ledger, new_pos, head_pos, visited);

        let len_rest = rest_states.len() - 1;
        // Move all their indexes forward len(states) because we are adding states to the beginning
        for state in rest_states.iter_mut() {
            for transition in state.t.iter_mut() {
                transition.0 += states.len();
                match transition.1 {
                    Transition::MoveRelativeStateConsume(s, p, c) => transition.1 = Transition::MoveRelativeStateConsume(s + states.len(), p, c),
                    Transition::MoveRelativeState(s, p) => transition.1 = Transition::MoveRelativeState(s + states.len(), p),
                    _ => {}
                }
            }
        }
        states.append(&mut rest_states);
        
        let next_type = ledger.identify(&func_box.pixels[get_index(new_pos, func_box.width)]).unwrap(); 
        let end_index = states.len() - 1;
        // The end of the states before appending new branch
        let end_1 = end_index - len_rest - 1;
        // The beginning of new states added 
        let start_2 = end_index - len_rest;
        // Combine end of root 1 to the beggining of root 2
        let transition: Transition;
        match next_type{
            // If the next is a loop, it will handle consuming itself, and its first is capture
            // And it relies that it is moving from this root node so... yea
            ColorType::Loop(_) => {
                // If right now is a loop, then just epsilon to that badboy
                // Else it needs to do what is said above
                match cur_type {
                    ColorType::Loop(_) => {transition = Transition::Epsilon;},
                    _ => {transition = Transition::MoveRelativeState(0, Point {x: 0, y: 0});}
                }
            },
            _ => {match cur_type {
                // Glitch where it would reference the start_capture instead end of
                // Because the state is 3 long we need to add 3 if it's a loop
                ColorType::Loop(_) => {transition = Transition::MoveRelativeStateConsume(3, pos, next_type);},
                _ => {transition = Transition::MoveRelativeStateConsume(0, pos, next_type);},
            }},
        }; 
        states[end_1].t.push((start_2, transition));
            
    }
    return states; 
}

impl FSM {
    pub fn do_machine(&self, g: Glyph, p: Picture){
    
    }
}

/// A definition of a function
#[derive(Debug, Clone)]
pub struct Definition{
    pub instructions: Vec<FSM>,
    pub identifier: GlyphType,
}

#[derive(Debug, Clone)]
pub struct Identified{
    glyph: Glyph,
    pub identifier: GlyphType,
    input: Vec<Color>,
    output: Vec<Color>,
    // flowin: i32
    // flowout: i32
    // These will be the id's in the identified vector of the flow
}

/// Identifiers for definition glyphs, like a function name
#[derive(Debug, Copy, Clone)]
pub enum GlyphType {
    Dir,
    Add,
    Mul,
    Div,
    Sub,
    Assign,
    Pic,
    CustomFunction(usize),
}

const BUILTIN_FILE_NAMES: [&str; 4] = [
    "symbols/add.png",
    "symbols/mul.png",
    "symbols/sub.png",
    "symbols/div.png",
];

pub struct DefinitionLedger {
    d: Vec<Definition>, 
}

impl DefinitionLedger {
    /// Checks to see if a glyph fits the definition
    pub fn identify(&self, glyph: &Glyph, pic: &Picture) -> Option<Identified> {

        // up left of the glyph
        let mut min = Point{x: i32::MAX, y: i32::MAX};
        for (p, _) in glyph.pixels.iter() {
            if p.x <= min.x && p.y <= min.y {
                min = *p;
            }
        }
        for definition in self.d.iter() {
 
        }
        // Checks the glyph fitsx
        // Checks the inputs fit
        // Checks the outputs fit
        // Checks that all the infinitley expandable things are equal where they need to be
        todo!();
    }

    pub fn load_pic_glyph(&mut self) {
        let pic = open_pic("symbols/pic.png");
        let mut definition = create_definition(pic, vec![], vec![]);
        definition.identifier = GlyphType::Pic; 
        self.d.push(definition);
    }

    pub fn load_dir(&mut self) {
        let mut pic = open_pic("symbols/dir.png");
        let glyphs = gather_glyphs(&pic, vec![Color{r: 255, g: 255, b: 0}]);
        let mut innards = Picture{pixels: vec![], width: -1, height: -1};
        for g in glyphs {
            let i = self.identify(&g, &pic);
            if i.is_some() {
                let identified = i.unwrap();
                match identified.identifier {
                    GlyphType::Pic => {innards = isolate_pic_innards(identified, &mut pic);},
                    _ => {}
                }
            }
        }
        let mut definition = create_definition(innards, vec![Color{r: 127, g: 201, b: 255}], vec![Color{r: 127, g: 255, b: 142}]);
        definition.identifier = GlyphType::Dir;
        self.d.push(definition);
    }
}

/// Gets the contents of Picture Struct and returns them
/// Sets them to White in the original Picture as to avoid any problems
fn isolate_pic_innards(g: Identified, mut p: &mut Picture) -> Picture {
    let mut new_picture = Picture{pixels: vec![], width: -1, height: -1};
    new_picture.width = (g.glyph.b.l_right.x) - (g.glyph.b.u_left.x + 1);
    new_picture.height = (g.glyph.b.l_right.y) - (g.glyph.b.u_left.y + 1);
    for j in (g.glyph.b.u_left.y + 1)..=(g.glyph.b.l_right.y - 1) {
        for i in (g.glyph.b.u_left.x + 1)..=(g.glyph.b.l_right.x - 1) {
        new_picture.pixels.push(p.pixels[get_index(Point{x: i, y: j}, p.width)]);
        }
    }
    println!{"{}, {}", new_picture.width, new_picture.height}
    return new_picture;
}
