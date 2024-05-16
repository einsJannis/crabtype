extern crate crossterm;
extern crate ratatui;
extern crate serde;
extern crate rand;

use std::{convert::TryFrom, env::{self, Args}, io::stdout, path::{Path, PathBuf}, str::FromStr};
use game::{GameMode, Stats};
use crate::game::Game;

mod game;
mod terminal;
mod round;
mod end;

const HELP_TEXT: &'static str = "crabtype [option...] Options: --gamemode|-g <file> ; --user|-u <file>";

fn main() {
    let mut args = env::args();
    let mut game_mode = GameMode::default();
    let mut stats_path = PathBuf::from_str("default.toml").unwrap();
    let mut stats = Stats::load_or_default(&stats_path);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-g"|"--gamemode" => {
                let Some(arg) = args.next().map(|it| PathBuf::from_str(&it).unwrap()) else { game_mode = GameMode::from(HELP_TEXT); break };
                game_mode = GameMode::load(&arg);
            }
            "-u"|"--user" => {
                let Some(arg) = args.next().map(|it| PathBuf::from_str(&it).unwrap()) else { game_mode = GameMode::from(HELP_TEXT); break };
                stats = Stats::load_or_default(&arg);
            }
            _ => {}
        }
    }
    let game = Game { game_mode, stats_path, stats };
    let game_stage = game.next_round();
    let mut terminal = terminal::Terminal::new(stdout()).unwrap();
    terminal.run(game_stage)
}
