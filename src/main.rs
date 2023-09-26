mod glyph;
use crate::glyph::*;


fn main() {
    let path = "symbols/dir.png";
    let image = open_image(path);

    let mut used_colors: Vec<Color> = vec![Color{r: 255, b: 0, g: 0}, Color{r: 0, b: 255, g: 0}, Color{r: 0, b: 0, g: 255}];
    for glyph in gather_glyphs(image, used_colors) {
        println!("glyph color {:?}", glyph.color);
        println!("glyph length {}", glyph.pixels.len());
        println!("glyph pixels {:?}", glyph.pixels);
    }
}
