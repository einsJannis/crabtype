use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use crate::game::{Game, HandleKeyEvent, Stage};

pub struct EndStage {
    pub game: Game,
    pub tpm: f64,
    pub wpm: f64
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
        frame.render_widget(
            Paragraph::new(format!("TPM: {:.2} WPM: {:.2}", self.tpm, self.wpm)).centered(),
            Rect::new(0,parent_size.height/2, parent_size.width, 1)
        );
    }
}

