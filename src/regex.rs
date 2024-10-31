use crate::picture::Picture;

/// Represents an individual state in the NFA
/// The last arguement of every state is the index to the next state
/// Consume takes a char that it will try to match
/// Move takes a state index to move to
/// Turn takes a 2d vector to turn to
/// Split takes two states that it coudld go to for free
/// Match represents the matching state
#[derive(PartialEq)]
pub enum State {
    Consume([u8; 3]), 
    Moveto(usize),
    Turn([i32; 2]),
    Split(usize),
    Match(),
}

/// Builder method for RegexBuilder
/// Builds by appending to the end so the regex is read right -> left
struct RegexBuilder {
    s: Vec<(State, usize)>
}

impl RegexBuilder{

    /// Match is the final matching state
    fn new() -> Self {
        return RegexBuilder { s: vec![(State::Match(), 0)]}
    }

    /// C is the color you are trying to match
    fn consume(&mut self, c: [u8; 3], i: usize) -> &mut Self {
        self.s.push((State::Consume(c), i));
        self
    }

    /// RS is the reference state you are moving to
    fn moveto(&mut self, rs: usize, i: usize) -> &mut Self {
        self.s.push((State::Moveto(rs), i));
        self
    }

    /// DX is the vcetor in which you are moving
    fn turn(&mut self, dx: [i32; 2], i: usize) -> &mut Self {
        self.s.push((State::Turn(dx), i));
        self
    }

    /// i and j are both indices of states
    fn split(&mut self, i: usize, j: usize) -> &mut Self {
        self.s.push((State::Split(j), i));
        self
    }

    /// Builds into Regex 
    fn build(self) -> Regex {
        Regex { s: self.s.into_boxed_slice() }
    }
}


pub struct Regex {
    s: Box<[(State, usize)]>
}

impl Regex {
    pub fn new() -> Self {
        let rb = RegexBuilder::new();

        rb.build()
    }
    pub fn from_pic(p: Picture) -> Self {
        let rb = RegexBuilder::new();
        rb.build()
    }
    
    /// Runs the regex from the x,y point in the given picture
    pub fn id(&self, p: Picture, mut x: i32, mut y: i32) -> bool {
            
    }

}
