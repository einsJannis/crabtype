use std::path::Path;
use std::time::Instant;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Paragraph, Wrap};
use crate::end::EndStage;
use crate::game::{Game, HandleKeyEvent, Stage};

pub struct RoundStage {
    game: Game,
    text: usize,
    error: String,
    index: usize,
    start_time: Option<Instant>
}

impl Game {
    pub fn next_round(self) -> RoundStage {
        RoundStage {
            text: self.game_mode.next_text(),
            game: self,
            error: Default::default(),
            start_time: Default::default(),
            index: Default::default()
        }
    }
}

impl RoundStage {
    fn end(mut self) -> EndStage {
        let elapsed = self.start_time.unwrap().elapsed();
        self.game.stats.attempts += 1;
        self.game.stats.tpm.add_value(
            60000.0 * (self.index as f64) /
                    elapsed.as_millis() as f64, self.game.stats.attempts);
        self.game.stats.wpm.add_value(
            60000.0 * (self.text().chars().filter(|it| *it == ' ').count() + 1) as f64 /
                                    elapsed.as_millis() as f64, self.game.stats.attempts);
        let failed_to_save =
            if let Ok(_) = self.game.stats.save(Path::new("stats.toml")) {
                false
            } else {true};
        EndStage {
            game: self.game,
            failed_to_save
        }
    }
    fn text(&self) -> &str {
        self.game.game_mode.text(self.text)
    }
}

impl HandleKeyEvent for RoundStage {
    fn handle_key<'b>(mut self: Box<Self>, key_event: KeyEvent) -> Option<Box<(dyn Stage + 'b)>> where Self: 'b {
        match key_event.code {
            KeyCode::Char(mut c) => {
                if self.start_time.is_none() {
                    self.start_time = Some(Instant::now());
                }
                if self.text().chars().nth(self.index).map(|it| it == c).unwrap_or(false) && self.error.is_empty() {
                    self.index += 1;
                } else {
                    if c == ' ' { c = '_' }
                    self.error.push(c)
                }
                if self.index == self.text().len() {
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

impl Stage for RoundStage {
    fn draw(&self, frame: &mut Frame) {
        let outer_size = frame.size();
        let x = outer_size.width * 20 / 100;
        let width = outer_size.width-(2*x);
        let height = ((self.text().len() + self.error.len())/(width as usize)) as u16 + 1;
        let y = (outer_size.height - height)/2;
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(&self.text()[..self.index], Style::new().fg(Color::Green)),
            Span::styled(&self.error, Style::new().fg(Color::Red)),
            Span::styled(&self.text()[self.index..], Style::new().fg(Color::Gray))
        ])).wrap(Wrap::default()), Rect::new(x, y, width, height));
    }
}
