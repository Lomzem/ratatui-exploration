mod app;
use app::App;
mod assignment;
use assignment::Assignment;

use color_eyre;
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = init_terminal()?;

    let items: Vec<Assignment> = vec![
        Assignment {
            course: "course1",
            name: "assignment1",
        },
        Assignment {
            course: "course2",
            name: "assignment2",
        },
        Assignment {
            course: "course3",
            name: "assignment3",
        },
    ];

    App::new(items);

    return Ok(());
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    crossterm::terminal::enable_raw_mode()?;
    stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    return Ok(terminal);
}
