mod glyph;
use crate::glyph::*;


fn main() {
    let path = "a.png";
    let image = open_image(path);

    let mut used_colors: Vec<Color> = vec![Color{r: 255, b: 0, g: 0}, Color{r: 0, b: 255, g: 0}, Color{r: 0, b: 0, g: 255}];
    let glyphs = gather_glyphs(image, used_colors);
    let builtin_list = load_builtin_glyphs();
    let identified =  identify_glyphs(builtin_list, glyphs);
    for iden in identified {
        println!("{:?}", iden.identifier);
    }
}
