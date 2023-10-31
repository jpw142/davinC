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
    let ledger = ColorLedger{
        inputs: vec![],
        outputs: vec![],
        function: Color { r: 255, g: 127, b: 248}
    };
    let glyphs = gather_glyphs(&image2, ledger);
    let def_ledger = DefinitionLedger{d: vec![definition]};
    for g in glyphs {
        println!("{:?}", def_ledger.identify(&g, &image2));
    }
}
