use std::fs::File;
use std::io::stdout;
use std::path::Path;
use ratatui::prelude::*;
use crate::game::{Game, Stage};

mod game;
mod terminal;
mod round;
mod end;

fn main() {
    let mut terminal = terminal::Terminal::new(stdout()).unwrap();
    let GAME_WHEN_PROMPT_LOADING_FAILED: Game = Game {
        text: Vec::from(["Something is wrong with your prompts.txt".into()]),
        tpm: Default::default(),
        wpm: Default::default()
    };
    let game = if let Ok(mut prompts_file) = File::open(Path::new("prompts.txt")) {
        Game::try_from(prompts_file).unwrap_or(GAME_WHEN_PROMPT_LOADING_FAILED)
    } else {
        GAME_WHEN_PROMPT_LOADING_FAILED
    };

    let game_stage = game.next_round();
    terminal.run(game_stage)
}
