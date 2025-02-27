use macroquad::prelude::*;
use shared::{utils::Player, ai::Robot, utils::check_winner};

pub struct Game {
    grid: [[Option<Player>; 3]; 3],
    current_player: Player,
    game_over: bool,
    winner: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: [[None; 3]; 3],
            current_player: Player::X,
            game_over: false,
            winner: None,
        }
    }

    pub fn update(&mut self, robot: &mut Robot) {
        if self.game_over {
            return; // Блокируем обновление, если игра завершена
        }

        if self.current_player == Player::O {
            let (x, y) = robot.make_move(&self.grid);
            self.make_move(x, y);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let cell_size = 100.0;
            let start_x = screen_width() / 2.0 - 1.5 * cell_size;
            let start_y = screen_height() / 2.0 - 1.5 * cell_size;

            let x = ((mouse_x - start_x) / cell_size) as usize;
            let y = ((mouse_y - start_y) / cell_size) as usize;

            if x < 3 && y < 3 && self.grid[y][x].is_none() {
                self.make_move(x, y);
            }
        }
    }

    pub fn draw(&self) {
        clear_background(WHITE);

        let cell_size = 100.0;
        let start_x = screen_width() / 2.0 - 1.5 * cell_size;
        let start_y = screen_height() / 2.0 - 1.5 * cell_size;

        // Draw grid
        for i in 0..=3 {
            draw_line(
                start_x + i as f32 * cell_size,
                start_y,
                start_x + i as f32 * cell_size,
                start_y + 3.0 * cell_size,
                2.0,
                BLACK,
            );

            draw_line(
                start_x,
                start_y + i as f32 * cell_size,
                start_x + 3.0 * cell_size,
                start_y + i as f32 * cell_size,
                2.0,
                BLACK,
            );
        }

        // Draw X and O
        for y in 0..3 {
            for x in 0..3 {
                if let Some(player) = self.grid[y][x] {
                    let pos_x = start_x + x as f32 * cell_size + cell_size / 2.0;
                    let pos_y = start_y + y as f32 * cell_size + cell_size / 2.0;

                    match player {
                        Player::X => {
                            draw_text("X", pos_x - 10.0, pos_y + 10.0, 40.0, RED);
                        }
                        Player::O => {
                            draw_text("O", pos_x - 10.0, pos_y + 10.0, 40.0, BLUE);
                        }
                    }
                }
            }
        }

        // Draw winner message
        if self.game_over {
            let text = match self.winner {
                Some(Player::X) => "X wins!",
                Some(Player::O) => "O wins!",
                None => "It's a draw!",
            };

            let text_width = measure_text(text, None, 50, 1.0).width;
            draw_text(
                text,
                screen_width() / 2.0 - text_width / 2.0,
                screen_height() / 2.0,
                50.0,
                DARKGRAY,
            );
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize) {
        if self.grid[y][x].is_none() && !self.game_over {
            self.grid[y][x] = Some(self.current_player);
            self.check_game_over();
            self.current_player = if self.current_player == Player::X {
                Player::O
            } else {
                Player::X
            };
        }
    }

    fn check_game_over(&mut self) {
        if let Some(winner) = check_winner(&self.grid) {
            self.winner = Some(winner);
            self.game_over = true;
        } else if self.grid.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
            self.game_over = true; // Ничья
        }
    }

    #[allow(dead_code)] // helpful in the future
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    #[allow(dead_code)]
    pub fn get_winner(&self) -> Option<Player> {
        self.winner
    }
}