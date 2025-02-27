use crate::utils::Player;
use rand::Rng;

pub struct Robot;

impl Robot {
    pub fn new() -> Self {
        Self
    }

    pub fn make_move(&self, grid: &[[Option<Player>; 3]; 3]) -> (usize, usize) {
        loop {
            let x = rand::thread_rng().gen_range(0..3);
            let y = rand::thread_rng().gen_range(0..3);
            if grid[y][x].is_none() {
                return (x, y);
            }
        }
    }
}