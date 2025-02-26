#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

pub fn check_winner(grid: &[[Option<Player>; 3]; 3]) -> Option<Player> {
    // Check rows
    for row in grid {
        if row[0].is_some() && row[0] == row[1] && row[1] == row[2] {
            return row[0];
        }
    }

    // Check columns
    for col in 0..3 {
        if grid[0][col].is_some() && grid[0][col] == grid[1][col] && grid[1][col] == grid[2][col] {
            return grid[0][col];
        }
    }

    // Check diagonals
    if grid[0][0].is_some() && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
        return grid[0][0];
    }
    if grid[0][2].is_some() && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
        return grid[0][2];
    }

    None
}