use std::io::{Write, stdout};
use std::{io, thread, time};

use crossterm::{
    event::{KeyCode, KeyEvent},
    style::Stylize
};
use rand::seq::SliceRandom;

use crate::{
    board::Board,
    direction::Direction,
    snake::{
        Snake,
        SNAKE_INIT_SIZE
    },
    point::{
        Point,
        generate_point
    },
    terminal_handler,
    config::Config,
    maze
};

pub struct Game {
    config: Config,
    board: Board,
    snake: Snake,
    maze: Option<Vec<Vec<bool>>>,
    has_user_lost: bool,
    apple: Option<Point>,
    has_vertical_debt: bool,
    has_user_quit: bool
}

impl Game {
    pub fn basic() -> Game {
        Game::new(8, 8)
    }
    
    pub fn new(width: u32, height: u32) -> Game {
        let config = Config::read();
        
        let board = Board::new(config.get_width(), config.get_height())
            .expect(
                &format!("Invalid Board configuration: ({width}, {height})")
            );
        let center = board.get_center();
        
        let maze: Option<Vec<Vec<bool>>>;
        let snake: Snake;
        
        if config.is_maze_enabled() {
            let actual_maze = maze::build_maze(&board, config.get_maze_freedom());
            snake = Game::generate_snake_spawn_between_maze(&actual_maze, &board);
            maze = Some(actual_maze);
        } else {
            maze = None;
            snake = Snake::from_center(center);
        }
        
        let mut game = Game {
            config,
            board,
            snake,
            maze,
            has_user_lost: false,
            apple: None,
            has_vertical_debt: true,
            has_user_quit: false
        };
        game.generate_apple();
        
        game
    }
    
    fn generate_snake_spawn_between_maze(maze: &Vec<Vec<bool>>, board: &Board) -> Snake {
        let suffix_free_count = Game::build_suffix_free_count(maze, board);
        let mut rng = rand::rng();
        let mut suitable_cells: Vec<Point> = vec![];
        
        for i in 0..board.get_height() {
            for j in 0..board.get_width() {
                if suffix_free_count[i][j] >= 4 {
                    suitable_cells.push(Point::new(j as i32, i as i32));
                }
            }
        }
        
        suitable_cells.shuffle(&mut rng);
        
        let tail = suitable_cells.get(0)
            .expect("Couldn't spawn a snake because of too tight maze. Try changing your configuration");
        
        Snake::from_segments(
            vec![
                Point::new(tail.get_x() + 1, tail.get_y()),
                tail.clone()
            ]
        )
    }
    
    fn build_suffix_free_count(maze: &Vec<Vec<bool>>, board: &Board) -> Vec<Vec<u32>> {
        let maze = maze;
        let mut suffix_free_count = vec![vec![0; board.get_width()]; board.get_height()];
        
        for i in 0..board.get_height() {
            for j in (0..board.get_width()).rev() {
                if !maze[i][j] {
                    let in_front_cell_count = suffix_free_count[i].get(j + 1).cloned().unwrap_or(0);
                    suffix_free_count[i][j] = in_front_cell_count + 1;
                }
            }
        }
        
        suffix_free_count
    }
    
    fn sleep(millis: u32) {
        thread::sleep(time::Duration::from_millis(millis as u64));
    }
    
    fn move_snake_forward(&mut self) {
        if self.snake.get_direction().is_vertical() {
            if self.has_vertical_debt {
                self.has_vertical_debt = false;
            } else {
                self.has_vertical_debt = true;
                self.snake.move_forward();
            }
        } else {
            self.has_vertical_debt = false;
            self.snake.move_forward();
        }
        self.validate_snake();
    }
    
    fn calculate_user_score(&self) -> u32 {
        (self.snake.get_segments().len() as u32).saturating_sub(SNAKE_INIT_SIZE)
    }
    
    fn grow_snake(&mut self) {
        self.snake.grow();
        self.validate_snake();
    }
    
    fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }
    
    fn validate_snake(&mut self) {
        let head = self.snake.get_head();
        
        self.has_user_lost = {
            head.get_x() >= self.board.get_width() as i32 ||
            head.get_y() >= self.board.get_height() as i32 ||
            head.get_x() < 0 || head.get_y() < 0 ||
            self.snake.is_self_collision() ||
            self.has_snake_hit_wall()
        }
    }
    
    fn has_snake_hit_wall(&self) -> bool {
        if let Some(maze) = &self.maze {
            let head = self.snake.get_head();
            return maze[head.get_y() as usize][head.get_x() as usize];
        }
        false
    }
    
    fn enable_game_mode(&self) -> io::Result<()> {
        terminal_handler::enable_raw_mode()?;
        terminal_handler::prepare_screen()?;
        Ok(())
    }
    
    fn disable_game_mode(&self) -> io::Result<()> {
        terminal_handler::show_cursor()?;
        terminal_handler::disable_raw_mode()?;
        Ok(())
    }
    
    fn get_direction_from_key_event(&self, key_event: KeyEvent) -> Option<Direction> {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('w') => Some(Direction::UP),
            KeyCode::Down | KeyCode::Char('s') => Some(Direction::DOWN),
            KeyCode::Left | KeyCode::Char('a') => Some(Direction::LEFT),
            KeyCode::Right | KeyCode::Char('d') => Some(Direction::RIGHT),
            _ => None
        }
    }
    
    fn get_user_input(&mut self) -> io::Result<Option<Direction>> {
        let key_event_option = terminal_handler::get_key_event()?;
        
        match key_event_option {
            Some(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    self.has_user_quit = true;
                    return Ok(None);
                }
                
                let new_direction = self.get_direction_from_key_event(key_event);
                Ok(new_direction)
            }
            None => {
                Ok(None)
            }
        }
    }

    fn generate_apple(&mut self) {
        if self.apple.is_some() {
            return;
        }
        
        loop {
            let apple_point = generate_point(self.board.get_width() as u32, self.board.get_height() as u32);
            
            assert!(apple_point.get_x() >= 0 && apple_point.get_y() >= 0,
                "Generated point for apple spawning has negative coordinates!");
            
            let apple_x = apple_point.get_x() as usize;
            let apple_y = apple_point.get_y() as usize;
            
            if !self.snake.get_segments().contains(&apple_point) {
                if let Some(maze) = &self.maze && maze[apple_y][apple_x] {
                    continue;
                }
                self.apple = Some(apple_point);
                break;
            }
        }
    }
    
    fn is_snake_head_on_apple(&self) -> bool {
        if let Some(apple_point) = self.apple {
            return *self.snake.get_head() == apple_point
        }
        false
    }
    
    fn hide_apple(&mut self) {
        self.apple = None;
    }
    
    fn quit(&self) -> io::Result<()> {
        self.disable_game_mode()?;
        if self.has_user_lost {
            println!("GAME OVER!");
        }
        Ok(())
    }
    
    pub fn start(&mut self) -> io::Result<()> {
        self.enable_game_mode()?;
        
        self.print_field()?;
        print!("Press P to start playing the game.\r\n");
        loop {
            let key = terminal_handler::get_key_event()?;
            if let Some(key_event) = key && key_event.code == KeyCode::Char('p') {
                break;
            }
            Game::sleep(200);
        }
        
        terminal_handler::clear_screen()?;
        self.print_field()?;
        
        self.main_loop()?;
        self.quit()?;
        
        Ok(())
    }
    
    fn main_loop(&mut self) -> io::Result<()> {
        
        loop {
            if self.has_user_lost {
                break;
            }
            
            self.print_field()?;
            self.generate_apple();
            Game::sleep(self.config.get_pause_time());
            
            let user_direction_option = self.get_user_input()?;
            
            if self.has_user_quit {
                break;
            }
            
            if let Some(user_direction) = user_direction_option {
                self.change_snake_direction(user_direction);
            }
            
            if self.is_snake_head_on_apple() {
                self.grow_snake();
                self.hide_apple();
            } else {
                self.move_snake_forward();
            }
        }
        
        Ok(())
    }
    
    fn build_field(&self) -> Vec<Vec<char>> {
        let mut field = vec![
            vec![' '; self.board.get_width()]; self.board.get_height()
        ];
        
        if let Some(maze) = &self.maze {
            for y in 0..self.board.get_height() {
                for x in 0..self.board.get_width() {
                    if maze[y][x] {
                        field[y][x] = '#';
                    }
                }
            }
        }
        
        if let Some(apple_point) = self.apple {
            field[apple_point.get_y() as usize][apple_point.get_x() as usize] = '$';
        }
        
        for seg in self.snake.get_segments() {
            field[seg.get_y() as usize][seg.get_x() as usize] = '*';
        }
        
        let head = self.snake.get_head();
        field[head.get_y() as usize][head.get_x() as usize] = '@';
        
        field
    }
    
    pub fn print_field(&self) -> io::Result<()> {
        terminal_handler::reset_cursor_position()?;
        
        let mut frame = String::new();
        
        for _ in 0..self.board.get_width() + 2 {
            frame.push('-');
        }
        frame.push_str("\r\n");
        
        let field = self.build_field();
        for row in field {
            frame.push('|');
            
            for cell in row {
                frame.push_str(&match cell {
                    '#' => format!("{}", cell.dark_red()),
                    '$' => format!("{}", cell.green()),
                    '@' | '*' => String::from(cell),
                    _ => String::from(" ")
                });
            }
            
            frame.push('|');
            frame.push_str("\r\n");
        }
        
        for _ in 0..self.board.get_width() + 2 {
            frame.push('-');
        }
        frame.push_str("\r\n");
        
        print!("{}", frame);
        
        print!("\r\nYour score: {}\r\n\r\n", self.calculate_user_score());
        
        stdout().flush()?;
        
        Ok(())
    }
}
