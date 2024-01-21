use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &mut App) {
    // TODO 分离各screen的绘制
    let main_screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(f.size())[0];

    let items: Vec<ListItem> = app
        .command_list
        .items
        .iter()
        .map(|command| {
            // let text = Paragraph::new(command.command.clone());
            ListItem::new(command.command.clone())
                .style(Style::default().fg(Color::White))
                .bg(Color::Black)
        })
        .collect();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Command List"))
        .highlight_symbol(">> ")
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White));
    f.render_stateful_widget(items, main_screen, &mut app.command_list.state);
}
