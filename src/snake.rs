use std::collections::VecDeque;

use crate::point::Point;
use crate::direction::Direction;

pub const SNAKE_INIT_SIZE: u32 = 2;

pub struct Snake {
    segments: VecDeque<Point>,
    direction: Direction
}

impl Snake {
    pub fn from_center(center: Point) -> Snake {
        Snake {
            segments: VecDeque::from(vec![center, center.change_by_x(-1)]),
            direction: Direction::RIGHT
        }
    }
    
    pub fn from_segments(segments: Vec<Point>) -> Snake {
        Snake {
            segments: VecDeque::from(segments),
            direction: Direction::RIGHT
        }
    }
    
    pub fn is_self_collision(&self) -> bool {
        self.segments.iter().skip(1).any(|segment| segment == self.get_head())
    }
    
    fn validate_direction_change(&self, direction: Direction) -> bool {
        match (self.get_direction(), direction) {
            (Direction::LEFT, Direction::RIGHT) => {
                false
            }
            (Direction::RIGHT, Direction::LEFT) => {
                false
            }
            (Direction::UP, Direction::DOWN) => {
                false
            }
            (Direction::DOWN, Direction::UP) => {
                false
            }
            _ => {
                true
            }
        }
    }
    
    pub fn change_direction(&mut self, direction: Direction) {
        if !self.validate_direction_change(direction) {
            return;
        }
        
        self.direction = direction;
    }
    
    pub fn move_forward(&mut self) {
        self.grow();
        self.segments.pop_back();
    }
    
    pub fn grow(&mut self) {
        let head = self.segments.front().expect("Snake has no body!");
        
        let new_head = match self.direction {
            Direction::DOWN => {
                Point::new(
                    head.get_x(),
                    head.get_y() + 1
                )
            }
            Direction::UP => {
                Point::new(
                    head.get_x(),
                    head.get_y() - 1
                )
            }
            Direction::RIGHT => {
                Point::new(
                    head.get_x() + 1,
                    head.get_y()
                )
            }
            Direction::LEFT => {
                Point::new(
                    head.get_x() - 1,
                    head.get_y()
                )
            }
        };
        
        self.segments.push_front(new_head);
    }
    
    pub fn get_segments(&self) -> &VecDeque<Point> {
        &self.segments
    }
    
    pub fn get_head(&self) -> &Point {
        self.segments.front().expect("Snake has no body!")
    }
    
    pub fn get_direction(&self) -> Direction {
        self.direction
    }
}
