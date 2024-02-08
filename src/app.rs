use crate::assignment::Assignment;
use ratatui::{prelude::*, widgets::*};

struct StatefulList<'a> {
    state: ListState,
    items: Vec<Assignment<'a>>,
    last_selected: Option<usize>,
}

impl StatefulList<'_> {
    fn with_items<'a>(items: Vec<Assignment>) -> StatefulList<'a> {
        return StatefulList {
            state: ListState::default(),
            items,
            last_selected: None,
        };
    }
}

pub struct App<'a> {
    items: StatefulList<'a>,
}

impl App<'_> {
    pub fn new<'a>(list_items: Vec<Assignment>) -> App<'a> {
        return App {
            items: StatefulList::with_items(list_items),
        };
    }
}

impl App<'_> {
    // pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
    //     loop {}
    // }
    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        return Ok(());
    }
}
