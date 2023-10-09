use crate::app::{App, AppResult, AppUIState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Global Key bindings

    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
            return Ok(());
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
            return Ok(());
        }

        KeyCode::Left => {
            app.decrement_counter();
            return Ok(());
        }
        // Other handlers you could add here.
        _ => {}
    }

    match app.state.ui {
        crate::app::AppUIState::Starting(_) => {
            match key_event.code {
                        // Counter handlers
                KeyCode::Enter => {
                    app.state.ui = AppUIState::Starting(crate::app::StartingState::Finished);
                    return Ok(());
                }
                _ => ()
            }
        },
        crate::app::AppUIState::ListArticles(_) => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                // Counter handlers
                KeyCode::Enter => {
                    app.try_select_article();
                }
                KeyCode::Up => {
                    app.article_up();
                }
                KeyCode::Down => {
                    app.article_down();
                }
                _ => (),
            }
        }
        crate::app::AppUIState::DisplayArticles(_) => todo!(),
    }
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Enter => {
            app.try_select_article();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
