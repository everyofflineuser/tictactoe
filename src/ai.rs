use crate::utils::Player;
use macroquad::rand;

pub struct Robot;

impl Robot {
    pub fn new() -> Self {
        Self
    }

    pub fn make_move(&self, grid: &[[Option<Player>; 3]; 3]) -> (usize, usize) {
        loop {
            let x = rand::gen_range(0, 3); // \u0413\u0435\u043d\u0435\u0440\u0430\u0446\u0438\u044f \u0441\u043b\u0443\u0447\u0430\u0439\u043d\u043e\u0433\u043e \u0447\u0438\u0441\u043b\u0430 \u043e\u0442 0 \u0434\u043e 2
            let y = rand::gen_range(0, 3); // \u0413\u0435\u043d\u0435\u0440\u0430\u0446\u0438\u044f \u0441\u043b\u0443\u0447\u0430\u0439\u043d\u043e\u0433\u043e \u0447\u0438\u0441\u043b\u0430 \u043e\u0442 0 \u0434\u043e 2
            if grid[y][x].is_none() {
                return (x, y);
            }
        }
    }
}