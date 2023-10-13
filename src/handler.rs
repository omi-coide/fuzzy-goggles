use crate::app::{App, AppResult, AppUIState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_resize(x:u16,y:u16,app:&mut App)-> AppResult<()> {
    match &app.state.ui {
        AppUIState::DisplayArticles(s) => {
            let mut s  = s.borrow_mut();
            s.reindex = true;
            Ok(())
        },
        _ => Ok(())
    }
}
/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Global Key bindings
// TODO 和这里逻辑类似，用handle_resize处理一下页面大小改变导致的串号问题，当改变大小时，从s.progress读取进度重新变成s.index
    match key_event.code {

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

    match &app.state.ui {
        crate::app::AppUIState::Starting(_) => {
            match key_event.code {
                // Counter handlers
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
        crate::app::AppUIState::DisplayArticles(s) => {
            let mut s= s.borrow_mut();
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    drop(s);
                    app.state.ui = AppUIState::Starting(crate::app::StartingState::Finished);
                }
                KeyCode::Enter => {
                    s.page_state.skip_draw = !s.page_state.skip_draw;
                } // TODO
                KeyCode::PageDown => {
                    let old_page_index = s.index;
                    s.index = (s.index + 1).clamp(0, s.pages.len().checked_sub(1).unwrap_or(0));
                    // let page = &s.pages[pageindex];
                    if s.index != old_page_index {
                        s.page_state.height=0;
                        s.page_state.width=0;
                    }
                    if s.pages.is_empty() {
                        s.bookmark = 0.0;
                    } else {
                        s.bookmark = (s.index+1).clamp(1, s.pages.len())as f32  / s.pages.len() as f32;
                    }
                }
                KeyCode::PageUp => {
                    let old_page_index = s.index;
                    s.index = (s.index.checked_sub(1).unwrap_or(0)).clamp(0, s.pages.len().checked_sub(1).unwrap_or(0));
                    // let page = &s.pages[pageindex];
                    if s.index != old_page_index {
                        s.page_state.height=0;
                        s.page_state.width=0;
                    }
                    if s.pages.is_empty() {
                        s.bookmark = 0.0;
                    } else {
                        s.bookmark = (s.index+1).clamp(1, s.pages.len())as f32  / s.pages.len() as f32;
                    } 
                }
                _ => ()
            }
        },
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
