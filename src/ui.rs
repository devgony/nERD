use std::borrow::BorrowMut;

use ratatui::widgets::StatefulWidget;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Size},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListDirection, Paragraph},
    Frame,
};
use tui_scrollview::ScrollView;

use crate::{app::App, builder::RectBuilder};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let schemas = app.get_schemas();
    let lists = schemas.iter().map(|schema| {
        let column_names = schema
            .clone()
            .column_defs
            .unwrap_or_default()
            .iter()
            .map(|column_def| column_def.name.clone())
            .collect::<Vec<_>>();

        List::new(column_names)
            .block(Block::bordered().title(schema.table_name.clone()))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
    });
    // a schema will take 20 of width and the number of column_defs of height

    let [title_rect, main_rect] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .areas(frame.area());

    let [sql_rect, erd_rect] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .areas(main_rect);

    let mut scroll_view = ScrollView::new(Size::new(erd_rect.width, 100));

    let mut rect_builder = RectBuilder::new(erd_rect.width);
    for list in lists {
        let rect = rect_builder.get_rect(list.len() + 2);

        scroll_view.render_widget(list.clone(), rect);
    }

    // let test_paragraph = Paragraph::new(Text::styled(
    //     "This is a long paragraph\na\nb\nc\nd\ne\nf\ng\nh\ni\nj",
    //     Style::default().fg(Color::Green),
    // ));

    // Layout::vertical([Constraint::Fill(1); 9])
    //     .split(erd_rect)
    //     .into_iter()
    //     .for_each(|cell| {
    //         scroll_view.render_widget(test_paragraph.clone(), *cell);
    //     });

    // Layout::horizontal([Constraint::Length(20); 4])
    //     .split(erd_rect)
    //     .into_iter()
    //     .for_each(|column| {
    //         Layout::vertical([Constraint::Fill(1); 9])
    //             // .margin(1)
    //             .split(*column)
    //             .into_iter()
    //             .for_each(|cell| {
    //                 // As widget is rendered, the layout should be larger to be scrollable
    //                 scroll_view.render_widget(list.clone(), *cell);
    //             });
    //     });

    // isn't there a way to render grid of widgets according to layout? not like manually rendering each widget by cordinates
    // let height = items.len() as u16 + 2; // +2 for the border
    // let test_rect = Rect::new(5, 5, 20, height);
    // scroll_view.render_widget(list.clone(), test_rect);

    // let test_rect = Rect::new(5 + 20 + 2, 5, 20, height);
    // scroll_view.render_widget(list, test_rect);

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

    // let sql_paragrah = Paragraph::new(Text::styled(
    //     app.sql_text.clone(),
    //     Style::default().fg(Color::Green),
    // ))
    // .block(Block::bordered().title("SQL pane"));

    // set sql_rect to the editor

    frame.render_widget(title, title_rect);
    frame.render_widget(&app.editor, sql_rect);
}
