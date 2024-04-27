#![feature(duration_millis_float)]

use std::fs::File;
use std::io::stdout;
use std::path::Path;
use ratatui::prelude::*;
use crate::game::{Game, RoundStage, Stage};

mod game;
mod terminal;

fn main() {
    let mut terminal = terminal::Terminal::new(stdout()).unwrap();
    let mut game = Game::try_from(File::open(Path::new("prompts.txt")).unwrap()).unwrap();
    let game_stage = game.next_round();
    terminal.run(game_stage)
}
