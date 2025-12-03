use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn change_by_x(&self, change: i32) -> Point {
        Point {
            x: self.x + change,
            ..*self
        }
    }

    pub fn change_by_y(&self, change: i32) -> Point {
        Point {
            y: self.y + change,
            ..*self
        }
    }
    
    pub fn get_x(&self) -> i32 {
        self.x
    }
    
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

pub fn generate_point(width: u32, height: u32) -> Point {
    let mut rng = rand::rng();
    
    let rand_x = rng.random_range(0..width) as i32;
    let rand_y = rng.random_range(0..height) as i32;
    
    Point::new(rand_x, rand_y)
}
