use anyhow::Result;
use color_eyre::owo_colors::OwoColorize;
use gluesql_core::data::Schema;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    style::{Color, Modifier, Style},
    widgets::{block::title, Block, Borders, List, ListDirection},
};
use tui_scrollview::ScrollViewState;
use tui_textarea::TextArea;

#[derive(Default)]
enum Mode {
    #[default]
    SQL,
    ERD,
}

#[derive(Default)]
pub struct App {
    pub mode: Mode,
    pub sql_text: String,
    pub scroll_view_state: ScrollViewState,
    pub state: AppState,
    pub editor: TextArea<'static>,
}

#[derive(Default, PartialEq)]
pub(crate) enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn new(sql_text: String) -> App {
        let mut editor = TextArea::new(vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
        ]);
        // editor.set_cursor_style(Style::default().white().on_blue());
        editor.set_block(
            Block::default()
                .border_style(Style::default())
                .borders(Borders::ALL)
                .title("SQL Editor".to_string()),
        );
        App {
            editor,
            sql_text,
            ..Default::default()
        }
    }

    pub fn get_schemas(&self) -> Vec<Schema> {
        self.sql_text
            .split(";")
            .filter_map(|sql| Schema::from_ddl(sql).ok()) // todo!() shuold show err message if cant parse
            .collect::<Vec<_>>()
    }

    pub fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match (&self.mode, event::read()?) {
            (_, Event::Key(key)) if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                match key.code {
                    Char('c') | Char('q') => {
                        self.quit();
                    }
                    Right => {
                        self.mode = Mode::ERD;
                    }
                    Left => {
                        self.mode = Mode::SQL;
                    }
                    Char('s') => {
                        self.sync();
                    }
                    _ => {}
                }
            }
            (Mode::SQL, Event::Key(key)) if key.kind == KeyEventKind::Press => {
                self.editor.input(key);
            }
            (Mode::ERD, Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                Char('j') | Down => self.scroll_view_state.scroll_down(),
                Char('k') | Up => self.scroll_view_state.scroll_up(),
                Char('h') | Left => self.scroll_view_state.scroll_left(),
                Char('l') | Right => self.scroll_view_state.scroll_right(),
                Char('f') | PageDown => self.scroll_view_state.scroll_page_down(),
                Char('b') | PageUp => self.scroll_view_state.scroll_page_up(),
                Char('g') | Home => self.scroll_view_state.scroll_to_top(),
                Char('G') | End => self.scroll_view_state.scroll_to_bottom(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    fn sync(&mut self) {
        // get text content from editor
        let sql_text = self.editor.lines().concat();

        self.sql_text = sql_text;
        // draw erd again

        // let schemas = self.get_schemas();
        // let lists = schemas.iter().map(|schema| {
        //     let column_names = schema
        //         .clone()
        //         .column_defs
        //         .unwrap_or_default()
        //         .iter()
        //         .map(|column_def| column_def.name.clone())
        //         .collect::<Vec<_>>();

        //     List::new(column_names)
        //         .block(Block::bordered().title(schema.table_name.clone()))
        //         .style(Style::default().fg(Color::White))
        //         .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        //         .highlight_symbol(">>")
        //         .repeat_highlight_symbol(true)
        //         .direction(ListDirection::TopToBottom)
        // });

        // should app contain frame?
        // just fire re-render event?
    }
}
