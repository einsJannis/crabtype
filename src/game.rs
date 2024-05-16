use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
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
    fn load(path: &Path) -> Result<Self, LoadError> {
        fn read(path: &Path) -> Result<String, std::io::Error> {
            let mut file = File::open(path)?;
            let mut string = String::new();
            file.read_to_string(&mut string)?;
            Ok(string)
        }
        toml::from_str(read(path).map_err(|_| LoadError::from(path))?.as_str()).map_err(|_| LoadError::from(path))
    }
    pub fn load_or_default(path: &Path) -> Self {
        Self::load(path).unwrap_or_default()
    }
    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = if path.exists() {
            File::create(path)?
        } else {
            File::create_new(path)?
        };
        file.write_all(toml::to_string_pretty(self).unwrap().as_bytes())
    }
}

pub struct LoadError {
    path: Box<str>
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Something is wrong with your {}", self.path))
    }
}

impl From<&Path> for LoadError {
    fn from(path: &Path) -> Self {
        Self { path: path.to_str().unwrap().to_string().into_boxed_str() }
    }
}

pub struct GameMode {
    pub text: Vec<Box<str>>,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::load(Path::new("prompts.txt"))
    }
}
    
impl GameMode {
    pub fn load(path: &Path) -> Self {
        fn inner(path: &Path) -> Result<GameMode, std::io::Error> {
            let file = File::open(path)?;
            let mut strings = Vec::new();
            for line in BufReader::new(file).lines() {
                strings.push(line?.into_boxed_str());
            }
            Ok(GameMode {
                text: strings,
            })
        }
        inner(path).map_err(|_| LoadError::from(path)).unwrap_or_else(Self::from)
    }
    pub fn next_text(&self) -> usize {
        thread_rng().gen_range(0..self.text.len())
    }
    pub fn text(&self, index: usize) -> &str {
        &self.text[index]
    }
}

impl From<LoadError> for GameMode {
    fn from(value: LoadError) -> Self {
        Self {
            text: Vec::from([format!("{}", value).into_boxed_str()])
        }
    }
}

impl From<&str> for GameMode {
    fn from(value: &str) -> Self {
        Self {
            text: Vec::from([value.into()])
        }
    }
}

pub struct Game {
    pub game_mode: GameMode,
    pub stats_path: PathBuf,
    pub stats: Stats,
}

