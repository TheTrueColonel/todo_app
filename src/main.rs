use std::{io, panic};
use std::error::Error;
use std::io::{Stderr, stderr};

use color_eyre::config::HookBuilder;
use color_eyre::eyre;
use color_eyre::eyre::Result;
use crossterm::{event, ExecutableCommand, execute};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

use crate::app::{App, CurrentScreen};
use crate::dbcontext::DbContext;
use crate::ui::ui;

mod ui;
mod app;
mod dbcontext;

fn main() -> Result<(), Box<dyn Error>> {
    DbContext::try_init_db()?;

    // Setup terminal
    let mut terminal = init_terminal()?;

    install_hooks()?;

    // Create app and run it
    let mut app = App::new();
    app.load_todo_items()?;
    let _res = run_app(&mut terminal, &mut app);

    // Restore terminal
    restore()?;
    execute!(terminal.backend_mut())?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(())
                    }
                    KeyCode::Char('d') => {
                        if app.todo_cursor_pos >= 0 {
                            _ = app.delete_todo_item();
                        }
                    }
                    KeyCode::Enter => {
                        if app.todo_cursor_pos == -1 {
                            app.current_screen = CurrentScreen::Adding;
                            app.currently_adding = true;
                        } else if app.todo_cursor_pos >= 0 {
                            _ = app.mark_item_complete();
                        }
                    }
                    KeyCode::Up => {
                        app.move_list_cursor_up();
                    }
                    KeyCode::Down => {
                        app.move_list_cursor_down();
                    }
                    _ => {}
                }
                CurrentScreen::Adding if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            app.save_todo_item()?;
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_adding = false;
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Char(value) => {
                            app.input.push(value);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stderr>>>{
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn restore() -> Result<()> {
    disable_raw_mode()?;
    stderr().execute(LeaveAlternateScreen)?;
    stderr().execute(DisableMouseCapture)?;

    Ok(())
}

fn install_hooks() -> Result<()> {
    let hook_builder = HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    // Convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        restore().unwrap();
        panic_hook(panic_info)
    }));

    // Convert from a color_eyre EyreHook to a erye ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        restore().unwrap();
        eyre_hook(error)
    }))?;

    Ok(())
}
