use std::io::stdout;
use crate::game::Game;

mod game;
mod terminal;
mod round;
mod end;

fn main() {
    let mut terminal = terminal::Terminal::new(stdout()).unwrap();
    let game = Game::default();
    let game_stage = game.next_round();
    terminal.run(game_stage)
}
