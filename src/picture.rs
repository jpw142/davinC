use image::GenericImageView;

/// Structe that stores Picture Information 
/// x 0 -> width then y 0 -> height
#[derive(Debug)]
pub struct Picture {
    pub p: Box<[[u8; 4]]>,
    pub w: usize,
    pub h: usize,
}

impl Picture {
    /// Creates a picture from an image file
    /// Can error if path is invalid
    pub fn from_path(s: &str) -> Self {
        let image = image::open(s).unwrap();
        let (w, h)= image.dimensions();
        let rgba = image.into_rgba8();
        let raw = rgba.pixels();
        let p: Vec<[u8; 4]> = raw.map(|p| {return p.0}).collect();
        return Picture { p: p.into_boxed_slice(), w: (w as usize), h: (h as usize)}
    }

    /// Returns point represented by Index
    pub fn point(&self, i: usize) -> (usize, usize) {
        return (i % self.w, i / self.h)
    }

    /// Returns index represented point
    pub fn index(&self, x: i32, y: i32) -> usize {
        assert!(y >= 0);
        assert!(x >= 0);
        return y as usize * self.h + x as usize
    }
}

