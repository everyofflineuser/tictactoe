use macroquad::{miniquad::conf::Icon, prelude::*};
mod game;

use game::Game;
use shared::ai::Robot;


fn conf() -> Conf {
    Conf {
        window_title: String::from("TIC-TAC-TOE but with matchmaking"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        window_resizable: false,
        icon: Some(Icon::miniquad_logo()),
        ..Default::default()
    }
}

#[macroquad::main(conf())]
async fn main() {
    let mut game = Game::new();
    let mut robot = Robot::new();

    loop {
        game.update(&mut robot);
        game.draw();

        next_frame().await;
    }
}