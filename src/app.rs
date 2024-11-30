use anyhow::Result;
use gluesql_core::data::Schema;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    style::Style,
    widgets::{Block, Borders},
};
use tui_scrollview::ScrollViewState;
use tui_textarea::TextArea;
use {serde::Serialize, std::fmt::Debug, thiserror::Error as ThisError};

#[derive(Default)]
enum Mode {
    #[default]
    SQL,
    ERD,
}

pub struct App {
    pub mode: Mode,
    pub scroll_view_state: ScrollViewState,
    pub state: AppState,
    pub editor: TextArea<'static>,
    // how to impl default ok Result?
    pub schemas: NerdResult<Vec<Schema>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            scroll_view_state: ScrollViewState::default(),
            state: AppState::default(),
            editor: TextArea::default(),
            schemas: Ok(Vec::new()),
        }
    }
}

#[derive(ThisError, Serialize, Debug, PartialEq, Default)]
pub enum Error {
    #[error("storage: {0}")]
    StorageMsg(String),

    #[default]
    #[error("storage: ")]
    Two,
}

pub type NerdResult<T, E = Error> = std::result::Result<T, E>;


#[derive(Default, PartialEq)]
pub(crate) enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn new(_sql_text: String) -> App {
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
            ..Default::default()
        }
    }

    pub fn get_schemas(&self) -> Option<Vec<Schema>> {
        let sql_text = self.editor.lines().concat();

        let schemas: Vec<Schema> = sql_text
            .split(";")
            .filter_map(|sql| Schema::from_ddl(sql).ok()) // todo!() shuold show err message if cant parse
            .collect();
        
        if schemas.is_empty() {
            None
        } else {
            Some(schemas)
        }
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
        // let sql_text = self.editor.lines().concat();

        // self.sql_text = sql_text;

        self.get_schemas();

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
