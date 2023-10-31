#![allow(dead_code)]
/// Basic color struct
/// rgb representation
/// u8, u8, u8
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Common color constants used throughout the program
pub const RED: Color = Color{r: 255, g: 0, b: 0};
pub const GREEN: Color = Color{r: 0, g: 255, b: 0};
pub const BLUE: Color = Color{r: 0, g: 0, b: 255};
pub const YELLOW: Color = Color{r: 255, g: 255, b: 0};
pub const WHITE: Color = Color{r: 255, g: 255, b: 255};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorType {
    Output(Color), // Set of output colors
    Input(Color), // Set of input colors
    Function, // Function color
    Loop(u8),
}

/// A ledger for all the colors the FSM should worry about
pub struct ColorLedger {
    pub inputs: Vec<Color>,
    pub outputs: Vec<Color>,
    pub function: Color,
}

impl ColorLedger {
    /// If the Ledger contains the color, it will return its domain
    /// If it doesn't contain the color, it will return None
    pub fn identify(&self, color: &Color) -> Option<ColorType> {
        if self.inputs.contains(color) {
            return Some(ColorType::Input(*color));
        }
        if self.outputs.contains(color) {
            return Some(ColorType::Output(*color));
        }
        if &self.function == color {
            return Some(ColorType::Function);
        }
        if (color.g == 0) && (color.b == 0) {
            return Some(ColorType::Loop(color.r));
        }
        return None;
    }
} 
