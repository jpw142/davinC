#![allow(dead_code)]
use crate::glyphdetect::picture::*;
use crate::glyphdetect::point::*;
use crate::glyphdetect::color::*;
use crate::glyphdetect::glyph::*;

/// A 2d finite state machine 
/// Contains a vector of states that contain transitions between themselves
/// The transitions are based upon the index in the vector of states
/// Always starts at index 0 and will end when reaching a state with 0 transitions
#[derive(Debug, Clone)]
pub struct FSM {
    pub states: Vec<State>,
}

impl FSM {
    /// Carries out a finite state machine on a glyph in a picture
    /// If it reaches a terminating state it will return a FSMResult
    /// If it does not reach a terminating state and runs out of transitions to go through it will
    /// return an Error
    pub fn do_machine(&self, g: Glyph, mut p: Picture) -> Result<FSMResult, FSMError> {
        // up left of the glyph
        let mut min = Point{x: i32::MAX, y: i32::MAX};
        for (p, _) in g.pixels.iter() {
            if p.x <= min.x && p.y <= min.y {
               min = *p;
            }
        }
        let history = vec![];
        let state_pos = vec![Point{x:0, y:0};self.states.len()];
        let color = p.pixels[get_index(min, p.width)];
        p.pixels[get_index(min, p.width)] = WHITE;
        let result = self.follow_machine(0, min, p, history, state_pos, -1);

        let mut fsm: FSMResult;
        match result {
            Ok(o) => {
                fsm = o;
                fsm.func.push((min, color));
                return Ok(fsm);
            },
            Err(e) => {
                return Err(e);
            }
 
        }
    }
    /// The recursive implementation for following a finite state machine
    /// cur_step: is the current state index in the state vector that we are at
    /// p: is the picture we are going through
    /// history: is the history of transitions we go through as to prevent infinite loops
    /// state_pos: is the vector of positions of states, when you enter a new state you store your
    /// position into the corresponding state position
    /// capture: designated whether we are currently in a capture length group, if -1 no capture
    /// group
    fn follow_machine(&self, cur_step: usize, cur_head: Point, mut p: Picture, history: Vec<(usize, Transition, usize)>, mut state_pos: Vec<Point>, capture: i16) -> Result<FSMResult, FSMError> {
        // If we reach a state with no transitions that is a terminal state, work our way back up
        if self.states[cur_step].t.len() == 0 {
            return Ok(FSMResult{func: vec![], inpt: vec![], outp: vec![], capt: vec![]})
        }
        state_pos[cur_step] = cur_head;
        let mut rest: Result<FSMResult, FSMError>;
        // Go through all transitions in the state to see if any have successful paths
        for (i, transition) in self.states[cur_step].t.iter() {
           match *transition {
               // Move to the place designated, relative to a state pos given
               Transition::MoveRelativeState(j, pos) => {
                   let mut my_history = history.clone();
                   my_history.push((cur_step, *transition, *i));
                   rest = self.follow_machine(
                       *i, 
                       state_pos[j] + pos, 
                       p.clone(), 
                       my_history, 
                       state_pos.clone(), 
                       capture
                       );
                   match rest {
                       Ok(o) => {return Ok(o)},
                       Err(_) => {continue}
                   }
               },
               // Do MoveRelativeState and consume the pixel there, if white it's wrong
               // We could make use of a color ledger here
               Transition::MoveRelativeStateConsume(j, pos, c) => {
                   // remove it from picture? and add this transiton to history
                   let mut my_history = history.clone();
                   let new_point = state_pos[j] + pos;
                   my_history.push((cur_step, *transition, *i));
                   if get_index(new_point, p.width) >= p.pixels.len() {
                       return Err(FSMError::Missing);
                   }
                   let color = p.pixels[get_index(new_point, p.width)];
                   if color == WHITE {
                       return Err(FSMError::Missing);
                   }
                   p.pixels[get_index(new_point, p.width)] = WHITE;
                   rest = self.follow_machine(
                       *i, 
                       new_point, 
                       p.clone(), 
                       my_history, 
                       state_pos.clone(), 
                       capture,
                       );
                   match rest {
                       Ok(o) => {
                           let mut fsm = o;
                           match c {
                               ColorType::Output(_) => {fsm.outp.push((new_point, color))},
                               ColorType::Input(_) => {fsm.inpt.push((new_point, color))},
                               ColorType::Function => {fsm.func.push((new_point, color))},
                               ColorType::Loop(_) => {continue;}
                           }
                           if capture != -1 {
                               let end = fsm.capt.len() - 1;
                               fsm.capt[end].1 += 1;

                            }
                            return Ok(fsm);
                        }
                        Err(_) => {
                            p.pixels[get_index(new_point, p.width)] = color;
                            continue
                        }
                    }
                    // add to consume list
                },
                // Free move to another state, with no other purpose
                Transition::Epsilon => {
                    let mut my_history = history.clone();
                    let end_history = my_history.len() - 1;
                    if my_history[end_history].1 == Transition::Epsilon && my_history[end_history].2 == cur_step {
                        return Err(FSMError::Missing);
                    }
                    my_history.push((cur_step, *transition, *i));
                    rest = self.follow_machine(
                        *i, 
                        cur_head, 
                        p.clone(), 
                        my_history, 
                        state_pos.clone(), 
                        capture
                        );
                    match rest {
                        Ok(o) => {return Ok(o)},
                        Err(_) => {continue},
                    }
                },
                // Starts a capture group which keeps track of how many pixels are consumed until
                // it ends
                Transition::CaptureType(_, j) => {
                    let mut my_history = history.clone();
                    my_history.push((cur_step, *transition, *i));

                    rest = self.follow_machine(
                        *i, 
                        cur_head, 
                        p.clone(), 
                        history.clone(), 
                        state_pos.clone(),
                        j as i16,
                        );
                    match rest {
                        Ok(o) => {
                            let mut fsm = o;
                            let end = fsm.capt.len() - 1;
                            fsm.capt[end].0 = j;
                            return Ok(fsm);
                        },
                        Err(_) => {continue;}
                    }
                },
                // Ends the capture group and henceforth ends the tracking of pixels
                Transition::EndCapture => {
                    let mut my_history = history.clone();
                    my_history.push((cur_step, *transition, *i));
                    rest = self.follow_machine(
                        *i, 
                        cur_head, 
                        p.clone(), 
                        history.clone(), 
                        state_pos.clone(),
                        -1,
                        );
                    match rest {
                        Ok(o) => {
                            let mut fsm = o;
                            fsm.capt.push((0,0));
                            return Ok(fsm);
                        },
                        Err(_) => {continue},
                    }
                },
            }
            
        }
        // If we get through all the states and none of them have successful tracks, return an
        // error
        return Err(FSMError::Missing);

    }
}

/// The result from carrying out a finite state machine upon a glyph
/// func contains all of the function pixels
/// inpt contains all the input colors
/// outp contains all of the output pictures
/// capt contains all the lengths that were captured
/// I need TODO, add what corresponding input color/output color they are
/// Like if a function takes blue and green and you input red and orange it should designate 
/// red -> blue and orange -> green
#[derive(Debug)]
pub struct FSMResult {
    func: Vec<(Point, Color)>,
    inpt: Vec<(Point, Color)>,
    outp: Vec<(Point, Color)>,
    capt: Vec<(u8, i32)>,
}

/// Error from carrying out a finite state machine
#[derive(Debug)]
pub enum FSMError {
    Missing, // I'm lazy so this is the catch all error right now
}


/// Identifiers for definition glyphs, like a function name for builtin functions
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

/// A definition of a function in all rotations and its name
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

/// Contains every single function definition for a program
pub struct DefinitionLedger {
    d: Vec<Definition>, 
}

impl DefinitionLedger {
    /// Checks to see if a glyph fits any definition
    pub fn identify(&self, glyph: &Glyph, pic: &Picture) -> Option<Identified> {

        // up left of the glyph
        let mut min = Point{x: i32::MAX, y: i32::MAX};
        for (p, _) in glyph.pixels.iter() {
            if p.x <= min.x && p.y <= min.y {
               min = *p;
            }
        }
        for definition in self.d.iter() {
            definition.instructions.iter().for_each(|f| {
                println!("{:?}", f.do_machine(glyph.clone(), pic.clone()));
                let result = f.do_machine(glyph.clone(), pic.clone());
                match result {
                    Ok(o) => {},
                    Err(_) => {}
                }
            });
        }
        
        // Checks the glyph fitsx
        // Checks the inputs fit
        // Checks the outputs fit
        // Checks that all the infinitley expandable things are equal where they need to be
        todo!();
    }

    /// Loads in the builtin function pic
    pub fn load_pic_glyph(&mut self) {
        let pic = open_pic("symbols/pic.png");
        let mut definition = create_definition(pic, vec![], vec![]);
        definition.identifier = GlyphType::Pic; 
        self.d.push(definition);
    }

    /// Loads in the builtin function dir
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
/// An abstract object that contains transitions to other indexes in its parent structure FSM
#[derive(Debug, Clone)]
pub struct State {
    pub t: Vec<(usize, Transition)>
}

/// The different transitions between states that are possible
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transition {
    MoveRelativeState(usize, Point), // Moves relative to a state
    MoveRelativeStateConsume(usize, Point, ColorType), // Move relative to a point from that state then consume
    Epsilon, // Change states for free
    CaptureType(ColorType, u8), // Starts a length capture group
    EndCapture, // Ends a capture group
}


/// Creates the definition when the dir is identified and the picture definition is found
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
    for pixel in func_box.pixels.iter_mut() {
        if ledger.identify(&pixel).is_none() {
            *pixel = WHITE;
        }
    }

    let mut definition = Definition{identifier: GlyphType::CustomFunction(0), instructions: vec![]};

    // Creates a definition for all 4 rotations
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
        // order them here
        definition.instructions.push(fsm);
        func_box = rotate(func_box);
    }
    return definition;
}

/// The recursive implentation of creating a finite state machine and its states
/// A glorified flood fill algorithm that depending on what pixel it's on and what pixel is
/// before/after adds states correspondingly
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
/// Gets the contents of Picture Struct and returns them
/// Sets them to White in the original Picture as to avoid any problems with things
fn isolate_pic_innards(g: Identified, p: &mut Picture) -> Picture {
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
