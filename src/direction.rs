// use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn is_vertical(&self) -> bool {
        match *self {
            Direction::UP | Direction::DOWN => {
                true
            }
            _ => {
                false
            }
        }
    }
    
    // pub fn generate_direction() -> Direction {
    //     let mut rng = rand::rng();
    //     let ind = rng.random_range(0..DIRECTIONS.len());
    //     DIRECTIONS[ind]
    // }
}
