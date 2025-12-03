use std::fs;
use serde::Deserialize;

const FILENAME: &str = "config.toml";

#[derive(Deserialize, Debug)]
pub struct Config {
    width: u32,
    height: u32,
    pause_time: u32,
    enable_maze: bool,
    maze_freedom: f64
}

impl Config {
    pub fn read() -> Config {
        let config_data = fs::read_to_string(FILENAME)
            .expect(&format!("Failed to read {FILENAME}. Check it again"));
        
        toml::from_str(&config_data)
            .expect(&format!("Failed to parse {FILENAME}. Check it again"))
    }
    
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    pub fn get_height(&self) -> u32 {
        self.height
    }
    
    pub fn get_pause_time(&self) -> u32 {
        self.pause_time
    }
    
    pub fn is_maze_enabled(&self) -> bool {
        self.enable_maze
    }
    
    pub fn get_maze_freedom(&self) -> f64 {
        self.maze_freedom
    }
}
