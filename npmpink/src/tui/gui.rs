use tui::{
    style::{Color, Style},
    symbols::DOT,
    text::Spans,
    widgets::{Block, Borders, Tabs},
};

use super::header::Header;

pub(super) fn build_tabs(header: &Header) -> Tabs {
    let titles = header.labels.iter().cloned().map(Spans::from).collect();
    Tabs::new(titles)
        .block(Block::default().title("header").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(DOT)
}

pub(super) fn build_sources_pane() {}
pub(super) fn build_packages_pane() {}
pub(super) fn build_help_pane() {}

#[cfg(test)]
mod tests {
    use crate::tui::header::Header;

    use super::build_tabs;

    #[test]
    pub fn test_build_tabs() {
        let header = Header::default().add_label("hello".into());

        build_tabs(&header);
    }
}
