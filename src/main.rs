mod app;
mod builder;
mod ui;

use anyhow::Result;
use app::AppState;
use color_eyre::config::HookBuilder;
use std::{
    error::Error,
    io::{self, stdout, Stderr},
    panic::{set_hook, take_hook},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

use crate::{app::App, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    init_error_hooks()?;
    let mut terminal = init_terminal()?;

    // should be enough complicated table exaples at least more than 6 columns
    let sql_text =
        "CREATE TABLE Employee (id int, name text, age int, salary int, address text, phone text);
CREATE TABLE Department (id int, name text, location text);
CREATE TABLE EmployeeDepartment (id int, employee_id int, department_id int);
CREATE TABLE EmployeeManager (id int, employee_id int, manager_id int);
CREATE TABLE Manager (id int, name text, age int, salary int, address text, phone text);
CREATE TABLE EmployeeSalary (id int, employee_id int, salary int);
CREATE TABLE EmployeeAddress (id int, employee_id int, address text);
CREATE TABLE EmployeePhone (id int, employee_id int, phone text);
CREATE TABLE DepartmentLocation (id int, department_id int, location text);
CREATE TABLE DepartmentManager (id int, department_id int, manager_id int);
CREATE TABLE ManagerSalary (id int, manager_id int, salary int);
CREATE TABLE ManagerAddress (id int, manager_id int, address text);
CREATE TABLE ManagerPhone (id int, manager_id int, phone text);
CREATE TABLE SalaryAddress (id int, salary_id int, address_id int);
CREATE TABLE SalaryPhone (id int, salary_id int, phone_id int);"
            .to_string();

    let mut app = App::new(sql_text);
    run_app(&mut terminal, &mut app)?;

    restore_terminal()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    while app.state != AppState::Quit {
        terminal.draw(|f| ui(f, app))?;

        app.handle_events()?;
    }

    Ok(())
}

fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
