mod regex;
mod picture;

use crate::{regex::Regex, picture::Picture};


fn main() {
    let r = Regex::new();
    print!("regex done");
    let p = Picture::from_path("C:\\Users\\jackw\\OneDrive\\Desktop\\family.jpg");
    println!("{:?}", p.p);
}


