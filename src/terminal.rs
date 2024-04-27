use std::io::Stdout;

use crossterm::{terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, event};
use ratatui::prelude::CrosstermBackend;
use crate::game::{HandleEvent, Stage};

pub struct Terminal {
    rterm: ratatui::Terminal<ratatui::backend::CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn run<S>(&mut self, initial_stage: S) where S: Stage {
        let mut stage: Option<Box<dyn Stage>> = Some(Box::new(initial_stage));
        while let Some(lstage) = stage {
            self.rterm.draw(|it| {
                if it.size().width > 20 && it.size().height > 5 {
                    lstage.draw(it)
                }
            }).unwrap();
            let event = event::read().unwrap();
            stage = lstage.handle_event(event);
        }
    }
    fn backend(&self) -> &CrosstermBackend<Stdout> {
        self.rterm.backend()
    }
    fn backend_mut(&mut self) -> &mut CrosstermBackend<Stdout> {
        self.rterm.backend_mut()
    }
    fn enter(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        self.backend_mut().execute(EnterAlternateScreen)?;
        Ok(())
    }
    fn leave(&mut self) -> std::io::Result<()> {
        disable_raw_mode()?;
        self.backend_mut().execute(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn new(mut stdout: std::io::Stdout) -> std::io::Result<Self> {
        let mut s = Self {
            rterm: ratatui::Terminal::new(CrosstermBackend::new(stdout))?,
        };
        s.enter()?;
        Ok(s)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.leave().unwrap();
    }
}
