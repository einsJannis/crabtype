use std::any::Any;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Stylize;
use ratatui::{style::Color, Frame};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use crate::game::{Game, HandleKeyEvent, Stage, Stat};

pub struct EndStage {
    pub game: Game,
    pub failed_to_save: bool,
}

impl HandleKeyEvent for EndStage {
    fn handle_key<'b>(mut self: Box<Self>, key_event: KeyEvent) -> Option<Box<(dyn Stage + 'b)>> where Self: 'b {
        match key_event.code {
            KeyCode::Enter => Some(Box::new(self.game.next_round())),
            _ => Some(self)
        }
    }
}

impl Stage for EndStage {
    fn draw(&self, frame: &mut Frame) {
        let parent_size = frame.size();
        let game = &self.game;
        let color = if (self.failed_to_save) {
            Color::Red
        } else {
            Color::White
        };
        frame.render_widget(
            Paragraph::new(
                format!(
                    "TPM: {:.2} average: {:.2}\nWPM: {:.2} average: {:.2}",
                    game.stats.tpm.last,
                    game.stats.tpm.average,
                    game.stats.wpm.last,
                    game.stats.wpm.average
                )
            ).centered().black().fg(color),
            Rect::new(0,parent_size.height/2, parent_size.width, 2)
        );
    }
}

