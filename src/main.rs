use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Stylize, Terminal},
    style::Color,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{Result, stdout};

enum Focus {
    Left,
    Right,
}

struct App {
    focus: Focus,
}

impl App {
    fn new() -> Self {
        Self { focus: Focus::Left }
    }

    fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Left => Focus::Right,
            Focus::Right => Focus::Left,
        };
    }
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(frame.area());

            let (left_color, right_color) = match app.focus {
                Focus::Left => (Color::Blue, Color::White),
                Focus::Right => (Color::White, Color::Blue),
            };

            let left_block = Block::default()
                .borders(Borders::ALL)
                .title("Left Pane")
                .fg(left_color);

            let right_block = Block::default()
                .borders(Borders::ALL)
                .title("Right Pane")
                .fg(right_color);

            frame.render_widget(
                Paragraph::new("Left Pane Content").block(left_block),
                layout[0],
            );
            frame.render_widget(
                Paragraph::new("Right Pane Content").block(right_block),
                layout[1],
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.toggle_focus();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
