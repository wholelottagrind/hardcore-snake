use crate::point::Point;

const MIN_SIDE: u32 = 8;
const MAX_SIDE: u32 = 1000;

pub struct Board {
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Option<Board> {
        if width < MIN_SIDE || height < MIN_SIDE || width > MAX_SIDE || height > MAX_SIDE {
            return None;
        }
        Some(Board { width: width as usize, height: height as usize })
    }

    pub fn get_center(&self) -> Point {
        Point::new(self.width as i32 / 2, self.height as i32 / 2)
    }
    
    pub fn get_width(&self) -> usize {
        self.width
    }
    
    pub fn get_height(&self) -> usize {
        self.height
    }
}
