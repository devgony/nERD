use std::borrow::BorrowMut;

use ratatui::{
    layout::{Constraint, Direction, Layout, Size},
    prelude::StatefulWidget,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListDirection, Paragraph},
    Frame,
};
use tui_scrollview::ScrollView;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
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

    let items = [
        "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6", "Item 7", "Item 8",
    ];

    let list = List::new(items)
        .block(Block::bordered().title("Table1"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let mut scroll_view = ScrollView::new(Size::new(erd_rect.width, 100));

    Layout::horizontal([Constraint::Length(20); 4])
        .split(erd_rect)
        .into_iter()
        .for_each(|column| {
            Layout::vertical([Constraint::Min(100); 9])
                .margin(1)
                .split(*column)
                .into_iter()
                .for_each(|y| {
                    // As widget is rendered, the layout should be larger to be scrollable
                    scroll_view.render_widget(list.clone(), *y);
                });
        });
    scroll_view.render(
        erd_rect,
        frame.buffer_mut(),
        app.scroll_view_state.borrow_mut(),
    );

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

    frame.render_widget(title, title_rect);
    frame.render_widget(sql_paragrah, sql_rect);
}
