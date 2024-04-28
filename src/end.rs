use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use crate::game::{Game, HandleKeyEvent, Stage, Stat};

pub struct EndStage {
    pub game: Game,
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
        frame.render_widget(
            Paragraph::new(format!(
                "TPM: {}\nWPM: {}", game.tpm, game.wpm)).centered(),
            Rect::new(0,parent_size.height/2, parent_size.width, 2)
        );
    }
}

