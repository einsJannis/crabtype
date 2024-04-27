use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use rand::{Rng, thread_rng};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};

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

pub struct Stats {
    average: f64,
    attempts: usize
}

pub struct Game {
    strings: Vec<Box<str>>,
}

impl Game {
    fn next_text(&self) -> &str {
        let index = thread_rng().gen_range(0..self.strings.len());
        self.strings[index].as_ref()
    }
    pub fn next_round(&self) -> RoundStage {
        RoundStage {
            text: self.next_text(),
            game: self,
            error: Default::default(),
            start_time: Default::default(),
            index: Default::default()
        }
    }
}

impl TryFrom<File> for Game {
    type Error = std::io::Error;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let mut strings = Vec::new();
        for line in BufReader::new(file).lines() {
            strings.push(line?.into_boxed_str());
        }
        Ok(Self {
            strings,
        })
    }
}

pub struct EndStage<'a> {
    game: &'a Game,
    tpm: f64,
}

impl<'a> HandleKeyEvent for EndStage<'a> {
    fn handle_key<'b>(mut self: Box<Self>, key_event: KeyEvent) -> Option<Box<(dyn Stage + 'a)>> where Self: 'b {
        match key_event.code {
            KeyCode::Enter => Some(Box::new(self.game.next_round())),
            _ => Some(self)
        }
    }
}

impl<'a> Stage for EndStage<'a> {
    fn draw(&self, frame: &mut Frame) {
        let parent_size = frame.size();
        frame.render_widget(Paragraph::new(format!("TPM: {}", self.tpm)), Rect::new(parent_size.width/2-5,parent_size.height/2, 10, 1));
    }
}

pub struct RoundStage<'a> {
    game: &'a Game,
    text: &'a str,
    error: String,
    index: usize,
    start_time: Option<Instant>
}

impl<'a> RoundStage<'a> {
    fn end(&mut self) -> EndStage<'a> {
        let elapsed = self.start_time.unwrap().elapsed();
        let tpm = (self.index as f64)/elapsed.as_millis_f64()*60000.0;
        //self.game.average = ((self.game.attempts-1) as f64 * self.game.average + tpm)/self.game.attempts as f64;
        EndStage {
            tpm,
            game: self.game
        }
    }
}

impl<'a> HandleKeyEvent for RoundStage<'a> {
    fn handle_key<'b>(mut self: Box<Self>, key_event: KeyEvent) -> Option<Box<(dyn Stage + 'a)>> where Self: 'b {
        match key_event.code {
            KeyCode::Char(mut c) => {
                if self.start_time.is_none() {
                    self.start_time = Some(Instant::now());
                }
                if self.text.chars().nth(self.index).map(|it| it == c).unwrap_or(false) && self.error.is_empty() {
                    self.index += 1;
                } else {
                    if c == ' ' { c = '_' }
                    self.error.push(c)
                }
                if self.index == self.text.len() {
                    return Some(Box::new(self.end()));
                }
            }
            KeyCode::Backspace => {
                if !self.error.is_empty() {
                    self.error.pop();
                } else {
                    self.index = self.index.saturating_sub(1);
                }
            }
            _ => {}
        }
        Some(self)
    }
}

impl<'a> Stage for RoundStage<'a> {
    fn draw(&self, frame: &mut Frame) {
        let outer_size = frame.size();
        let x = outer_size.width * 20 / 100;
        let width = outer_size.width-(2*x);
        let height = ((self.text.len() + self.error.len())/(width as usize)) as u16 + 1;
        let y = (outer_size.height - height)/2;
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(&self.text[..self.index], Style::new().fg(Color::Green)),
            Span::styled(&self.error, Style::new().fg(Color::Red)),
            Span::styled(&self.text[self.index..], Style::new().fg(Color::Gray))
       ])), Rect::new(x, y, width, height));
    }
}
