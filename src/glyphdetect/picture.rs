use image::io::Reader;
use crate::glyphdetect::color::*;
use crate::glyphdetect::point::*;

#[derive(Debug, Clone)]
pub struct Picture {
    pub pixels: Vec<Color>,
    pub width: i32,
    pub height: i32,
}

/// Opens image and given the file path is correct, returns the image in the picture format
pub fn open_pic(path: &str) -> Picture {
    let img = Reader::open(path).unwrap().decode().unwrap().to_rgb8();

    let width = img.width();
    let height = img.height();
    let data: Vec<u8>= img.into_raw();

    let mut ret_img: Picture = Picture{
        pixels: vec![],
        width: width as i32,
        height: height as i32,
    };

    for c in 0..(height * width) {
        let c3 = (c * 3) as usize;
        let color = Color { r: data[c3], g: data[c3 + 1], b: data[c3 + 2]}; 
        ret_img.pixels.push(color);
    }
    return ret_img;
}


/// Returns the subpicture of an area from that picture
/// the bounds are inclusive so (0,0) -> (1,1) would be a 2x2
/// the pixels still retain their points from the picture so... 
/// do with that what you may
pub fn subpicture(pic: &Picture, bounds: BoundingBox) -> Picture {
    let mut vec = vec![];
    for j in bounds.u_left.y..=bounds.l_right.y {
        for i in bounds.u_left.x..=bounds.l_right.x {
           vec.push(pic.pixels[get_index(Point{x: i, y: j}, pic.width)]) 
        }
    }
    return Picture { 
        pixels: vec, 
        width: (bounds.l_right.x - bounds.u_left.x) + 1, 
        height: (bounds.l_right.y - bounds.u_left.y) + 1, 
    }
}

pub fn rotate(pic: Picture) -> Picture {
    let n = pic.width;
    let m = pic.height;
    let mut new_picture = Picture{pixels: vec![WHITE; pic.pixels.len()], width: m, height: n};
    for i in 0..n {
        for j in 0..m {
            new_picture.pixels[get_index(Point{x: j, y:n - i - 1}, new_picture.width)] = pic.pixels[get_index(Point { x: i, y: j}, pic.width)];
        }
    }
    return new_picture;
}
