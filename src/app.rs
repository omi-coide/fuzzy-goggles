use std::{collections::HashMap, error, time::{Instant, Duration}};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub state: AppState,
}
#[derive(Debug, Clone)]
pub struct AppState {
    pub timer: Instant,
    pub ui: AppUIState
}
#[derive(Debug, Clone)]
pub enum AppUIState {
    Starting(StartingState),
    ListArticles(ListArticleState),
    DisplayArticles(ArticleState),
}
#[derive(Debug, Clone)]
pub struct ArticleState {
    pub pages: Vec<Page>,
    pub index: usize,
    pub crypt: HashMap<uuid::Uuid, bool>,
}
#[derive(Debug, Clone)]
pub enum StartingState {
    Finished,
    InProgress(usize),
}
#[derive(Debug, Clone)]
pub struct Page {
    lines: Vec<String>,
    height: usize,
    width: usize,
}
#[derive(Debug, Clone)]
pub struct ListArticleState {
    articles: Vec<String>
}

impl Default for AppUIState {
    fn default() -> Self {
        AppUIState::Starting(StartingState::InProgress(0))
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            timer: Instant::now(),
            ui: AppUIState::default() ,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            state: AppState::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()

    }

    /// Handles the tick event of the terminal.
    pub fn tick(state: &mut AppState) {
        if state.timer.elapsed() <= Duration::from_millis(100) {
            return;
        }
        state.timer = Instant::now();
        let current_state = state.clone();
        match current_state.ui {
            AppUIState::Starting(x) => App::tick_starting(x,state),
            AppUIState::ListArticles(x) => todo!(),
            AppUIState::DisplayArticles(_) => todo!(),
        };
    }
    fn tick_starting(s: StartingState, state: &mut AppState) {
        let next_state = match s {
            StartingState::Finished => AppUIState::ListArticles(ListArticleState{ articles: todo!() }),
            StartingState::InProgress(x) => {
                if x == 100 {
                    AppUIState::Starting(StartingState::Finished)
                } else {
                    AppUIState::Starting(StartingState::InProgress(x + 1))
                }
            }
        };
        state.ui = next_state;
    }
    fn tick_list(x:usize){
        if x>0 {
            todo!();
        }
    }
    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

use ratatui::widgets::StatefulWidget;

impl StatefulWidget for App{
    type State = AppState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        Self::tick(state);
    }
}