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

    let [title_rect, main_rect] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .areas(frame.area());

    let [sql_rect, erd_rect] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .areas(main_rect);

    let items = ["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];

    let list = List::new(items)
        .block(Block::bordered().title("Table1"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let table_rects = Layout::horizontal([Constraint::Length(20); 4])
        // .vertical_margin(1)
        // .horizontal_margin(2)
        .split(erd_rect)
        .into_iter()
        .for_each(|column| {
            Layout::vertical([Constraint::Min(50); 6])
                .margin(1)
                .split(*column)
                .into_iter()
                .for_each(|y| {
                    frame.render_widget(list.clone(), *y);
                });
        });
    // .flatten()
    // .collect::<Vec<&Rect>>();

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title =
        Paragraph::new(Text::styled("nERD", Style::default().fg(Color::Green))).block(title_block);

    let sql_paragrah = Paragraph::new(Text::styled(
        app.sql_text.clone(),
        Style::default().fg(Color::Green),
    ))
    .block(Block::bordered().title("SQL pane"));

    // should be node-graph widget
    let erd_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // let erd = Paragraph::new(Text::styled("Some ERD", Style::default().fg(Color::Green)))
    //     .block(erd_block);

    frame.render_widget(title, title_rect);
    frame.render_widget(sql_paragrah, sql_rect);
    // use scrollable widget instead of block
    frame.render_widget(Block::bordered().title("ERD pane"), erd_rect);
    // frame.render_widget(erd_block, columns[1]);
    // frame.render_widget(erd, columns[1]);

    // frame.render_widget(list, table_rect[0]);
}
