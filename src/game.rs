use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use rand::{Rng, thread_rng};
use ratatui::prelude::*;
use serde::{Deserialize, Serialize};

pub trait Stage: HandleEvent {
    fn draw(&self, frame: &mut Frame);
}

pub trait HandleEvent {
    fn handle_event<'a>(self: Box<Self>, event: Event) -> Option<Box<(dyn Stage + 'a)>> where Self: 'a;
}

pub trait HandleKeyEvent {
    fn handle_key<'a>(self: Box<Self>, key_event: KeyEvent) -> Option<Box<(dyn Stage + 'a)>> where Self: 'a;
}

impl<S:Stage + HandleKeyEvent> HandleEvent for S {
    fn handle_event<'a>(mut self: Box<Self>, event: Event) -> Option<Box<(dyn Stage + 'a)>> where Self: 'a {
        match event {
            Event::Key(event) => {
                if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
                    None
                } else {
                    self.handle_key(event)
                }
            },
            _ => Some(self)
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Stat {
    pub last : f64,
    pub average: f64,
}

impl Stat {
    pub fn add_value(&mut self, new: f64, attempts: usize) -> () {
        self.last = new;
        self.average += (new - self.average) / (attempts as f64) // trust me bro
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Stats {
    pub attempts: usize,
    pub tpm: Stat,
    pub wpm: Stat,
}

impl Stats {
    pub fn load_or_default(path: &Path) -> Self {
        Self::try_from(path).unwrap_or_default()
    }
    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = if path.exists() { File::create(path)? } else { File::create_new(path)?  };
        let error = file.write_all(toml::to_string_pretty(&self).unwrap().as_bytes());
        let error = if let Err(error) = error { format!("{}",error) } else { String::from("Ok") };
        Ok(())
    }
}

pub enum Either<TA,TB> {
    A(TA),
    B(TB),
}

impl<TA,TB> From<TA> for Either<TA,TB> {
    fn from(value: TA) -> Self {
        Self::A(value)
    }
}

impl TryFrom<&Path> for Stats {
    type Error = Either<std::io::Error,toml::de::Error>;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut file = File::open(path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        toml::from_str(&string).map_err(|err| Either::B(err))
    }
}

pub struct GameMode {
    pub text: Vec<Box<str>>,
}

impl Default for GameMode {
    fn default() -> Self {
        Self {
            text: Vec::from(["Something is wrong with your prompts.txt".into()]),
        }
    }
}
    
impl GameMode {
    pub fn load_or_default(path: &Path) -> Self {
        Self::try_from(Path::new("prompts.txt")).unwrap_or_default()
    }
    pub fn next_text(&self) -> usize {
        thread_rng().gen_range(0..self.text.len())
    }
    pub fn text(&self, index: usize) -> &str {
        &self.text[index]
    }
}

impl TryFrom<&Path> for GameMode {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let mut strings = Vec::new();
        for line in BufReader::new(file).lines() {
            strings.push(line?.into_boxed_str());
        }
        Ok(Self {
            text: strings,
            ..Default::default()
        })
    }
}

pub struct Game {
    pub game_mode: GameMode,
    pub stats: Stats,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            game_mode: GameMode::load_or_default(Path::new("prompts.txt")),
            stats: Stats::load_or_default(Path::new("stats.toml"))
        }
    }
}

