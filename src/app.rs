use gluesql_core::data::Schema;

// should we hold two state for sqls and schemas?
pub struct App {
    pub sql_text: String,
}

impl App {
    pub fn new(sql_text: String) -> App {
        App { sql_text }
    }

    pub fn get_schemas(&self) -> Vec<Schema> {
        self.sql_text
            .split(";")
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect::<Vec<_>>()
    }

    // pub fn sql_string(&self) -> String {
    //     self.sqls.join("\n")
    // }
}
