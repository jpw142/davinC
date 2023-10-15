mod glyphdetect;
use crate::glyphdetect::glyph::*;
use crate::glyphdetect::color::*;
use crate::glyphdetect::picture::*;


fn main() {
    let path = "b.png";
    let image = open_pic(path);
    create_definition(image, vec![], vec![]);

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
