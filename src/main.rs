#![allow(non_snake_case)]
mod glyphdetect;
use crate::glyphdetect::glyph::*;
use crate::glyphdetect::color::*;
use crate::glyphdetect::picture::*;
use crate::glyphdetect::fsm::*;


fn main() {
    let path = "b.png";
    let path2 = "c.png";
    let image = open_pic(path);
    let image2 = open_pic(path2);
    let definition = create_definition(image.clone(), vec![Color{r: 0, g: 148, b: 255}], vec![]);
    let glyphs = gather_glyphs(&image2, vec![Color{r: 255, g:127, b: 248}]);
    for g in glyphs {
        definition.instructions.iter().for_each(|f| {
            println!("{:?}", f.do_machine(g.clone(), image2.clone()));
        });
    }
    
  //  let mut colors_to_look_for: Vec<Color> = vec![
  //      Color{r: 255, b: 0, g:255}, // YELLOW -> Picture
  //      Color{r: 0, b: 255, g: 0},  // BLUE -> Functions
  //      Color{r: 0, b: 0, g: 255} // GREEN -> Program Flow
  //  ];
  //  let glyphs = gather_glyphs(&image, colors_to_look_for);
  //  let builtin_list = load_builtin_definitions();
  //  for iden in builtin_list {
  //      println!("{:?}", iden);
  //  }
}
