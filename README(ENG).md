# Tic-Tac-Toe but with matchmaking

## Game Description

This is an enhanced version of the classic game "Tic-Tac-Toe," where two players compete in a **series of 9 matches** against each other. The main goal is to **score more points than your opponent** by winning matches and strategically capturing cells on the global map.

### Key Features:
1. **9 Matches Against the Same Opponent**:
   - The game consists of 9 individual matches where two players compete against each other.
   - For each match won, the player earns **1 point**.

2. **Global Map**:
   - After each match, the captured cell on the 3x3 global map is marked with the winner's symbol (X or O).
   - If a player forms a **winning combination** (three cells in a row, column, or diagonal) on the global map, they earn an **additional point**.

3. **Game Outcome**:
   - The game ends after 9 matches.
   - The player with the **most points** wins (maximum of 10 points: 9 for match wins and 1 for a global combination).

---

## How to Play

### Classic Match Rules:
1. Players take turns placing **X** or **O** on a 3x3 grid.
2. The first player to align three of their symbols in a row (horizontally, vertically, or diagonally) wins the match.
3. If all cells are filled and no one wins, the match ends in a draw.

### Global Mechanics:
1. After each match, the captured cell on the 3x3 global map is marked with the winner's symbol.
2. If a player forms three cells in a row on the global map, they earn an **additional point**.
3. The game continues until all 9 matches are completed.

---

## Example Gameplay

1. **Match 1**:
   - Player X wins the match and captures cell (1, 1) on the global map.
   - Score: X - 1 point, O - 0 points.

2. **Match 2**:
   - Player O wins the match and captures cell (2, 2).
   - Score: X - 1 point, O - 1 point.

3. **Match 3**:
   - Player X wins the match and captures cell (3, 3).
   - Score: X - 2 points, O - 1 point.

4. **Global Map**:
   - If Player X captures cells (1, 1), (2, 2), and (3, 3), they earn an additional point for the diagonal.
   - Score: X - 3 points, O - 1 point.

5. **Outcome**:
   - After 9 matches, the players compare their scores. The player with the most points wins.

---

## Tools Used

### Programming Language:
- **Rust**: Chosen for its performance and safety. This is my first experience with Rust, and I am learning as I develop.

### Libraries:
- **Macroquad**: Used for creating the graphical interface and managing the game loop. It is a simple and convenient library for 2D games.
- **DeepSeek**: Used for searching information and assisting with coding. As a beginner, I actively use DeepSeek to learn Rust and solve complex problems.

### Additional Tools:
- **Git**: For version control and code management.
- **Cargo**: For dependency management and project building.

---

## How to Run the Game

1. Ensure you have Rust and Cargo installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/everyofflineuser/tictactoe.git
   ```
3. Navigate to the project directory:
   ```bash
   cd tictactoe
   ```
4. Run the game:
   ```bash
   cargo run
   ```

### OR

Simply download the game from the Releases tab (if available).

---

## Conclusion

This project is a great way to learn Rust and create an interesting game with unique mechanics. The main goal is to score more points than your opponent by winning matches and strategically capturing cells on the global map. If you have any suggestions or questions, feel free to reach out! 😊