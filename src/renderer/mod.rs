use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Clear},
    Frame, Terminal,
};

pub struct Renderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Renderer {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub fn render(&mut self, text: &str) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
            let block = Block::default().title("Block 3").borders(Borders::ALL);
            f.render_widget(block, chunks[2])
        })?;
        Ok(())
    }
}
