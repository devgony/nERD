use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListDirection, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let schemas = app.get_schemas();
    // a schema will take 20 of width and the number of column_defs of height

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main[1]);

    let erd_pane = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(20), Constraint::Max(0)])
        .split(panes[1]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled("nERD", Style::default().fg(Color::Green))).block(title_block);

    let sql_pane = Paragraph::new(Text::styled(
        app.sql_text.clone(),
        Style::default().fg(Color::Green),
    ))
    .block(Block::bordered());

    // should be node-graph widget
    let erd_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // let erd = Paragraph::new(Text::styled("Some ERD", Style::default().fg(Color::Green)))
    //     .block(erd_block);

    frame.render_widget(title, main[0]);
    frame.render_widget(sql_pane, panes[0]);
    // frame.render_widget(erd_block, columns[1]);
    // frame.render_widget(erd, columns[1]);

    let items = ["Item 1", "Item 2", "Item 3"];

    let list = List::new(items)
        .block(Block::bordered().title("Table1"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(list, erd_pane[0]);
}
