use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::time::{Duration, Instant};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use rand::{Rng, thread_rng};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};
use crate::round::RoundStage;

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
    text: Vec<Box<str>>,
}

impl Game {
    pub fn next_text(&self) -> usize {
        thread_rng().gen_range(0..self.text.len())
    }
    pub fn text(&self, index: usize) -> &str {
        &self.text[index]
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
            text: strings,
        })
    }
}

