use rand::{Rng, seq::SliceRandom};
use crate::{
    board::{Board}, point::{Point, generate_point}
};

pub fn build_maze(board: &Board, remove_chance: f64) -> Vec<Vec<bool>> {
    let mut maze = vec![vec![true; board.get_width()]; board.get_height()];
    
    perform_dfs(&mut maze, board);
    erode_dfs_maze(&mut maze, remove_chance);

    maze
}

fn erode_dfs_maze(maze: &mut Vec<Vec<bool>>, remove_chance: f64) {
    let mut rng = rand::rng();
    
    for row in maze.iter_mut() {
        for cell in row.iter_mut() {
            if *cell && rng.random_bool(remove_chance) {
                *cell = false;
            }
        }
    }
}

fn perform_dfs(maze: &mut Vec<Vec<bool>>, board: &Board) {
    let entry_point = generate_point(board.get_width() as u32, board.get_height() as u32);

    let mut stack = vec![entry_point];
    maze[entry_point.get_y() as usize][entry_point.get_x() as usize] = false;

    while let Some(last_point) = stack.last() {
        let (curr_x, curr_y) = (last_point.get_x() as usize, last_point.get_y() as usize);
        maze[curr_y][curr_x] = false;
        
        let mut is_leaf = true;

        for (new_x, new_y) in generate_random_neighbors(curr_x, curr_y) {
            if new_x < board.get_width() && new_y < board.get_height() &&
                    maze[new_y][new_x] &&
                    !is_node_next_to_visited(new_x, new_y, curr_x, curr_y, &maze, board) {
                stack.push(Point::new(new_x as i32, new_y as i32));
                is_leaf = false;
                break;
            }
        }
        
        if is_leaf {
            stack.pop();
        }
    }
}

fn get_neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x.saturating_sub(1), y),
        (x, y.saturating_sub(1)),
        (x + 1, y),
        (x, y + 1)
    ]
}

fn generate_random_neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    let mut neighbors = get_neighbors(x, y);

    let mut rng = rand::rng();
    neighbors.shuffle(&mut rng);

    neighbors
}

fn is_node_next_to_visited(x: usize, y: usize, parent_x: usize, parent_y: usize,
        maze: &Vec<Vec<bool>>, board: &Board) -> bool {
    let neighbors = get_neighbors(x, y);

    for (x_n, y_n) in neighbors {
        if x_n < board.get_width() && 
            y_n < board.get_height() &&
            (x_n, y_n) != (parent_x, parent_y) &&
            !maze[y_n][x_n] {
                return true;
        }
    }

    false
}
