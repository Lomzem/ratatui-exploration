pub struct Assignment<'a> {
    pub course: &'a str,
    pub name: &'a str,
}

impl Assignment<'_> {
    fn to_list_item(&self) -> ratatui::widgets::ListItem {
        let line = ratatui::prelude::Line::from(format!("{} - {}", self.course, self.name));
        return ratatui::widgets::ListItem::new(line);
    }
}
