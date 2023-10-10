use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    error, fs,
    time::{Duration, Instant},
};
const ARTICLE_DIR: &str = ".";
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.

pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub state: AppState,
    pub logo: Box<dyn ResizeProtocol>,
}
#[derive(Debug, Clone)]
pub struct AppState {
    pub timer: Instant,
    pub ui: AppUIState,
    pub articles: BTreeMap<String,String>
}
#[derive(Debug, Clone)]
pub enum AppUIState {
    Starting(StartingState),
    ListArticles(RefCell<ListArticleState>),
    DisplayArticles(ArticleState),
}


#[derive(Debug, Clone)]
pub struct ArticleState {
    pub article: String,
    pub bookmark: usize,                  // 阅读进度(0.0-1.0之间)
    pub progress: f32,          // 全局阅读进度
    pub mask_progress: f32,     // 页内动画进度
    pub crypt: HashMap<uuid::Uuid, bool>, //加密情况
}

impl ArticleState {
    fn tick(&mut self) {
        if self.mask_progress>= 0.999999 {
        }
        // match item {
        //     Control::Default
        //     | Control::NoBreakBegin
        //     | Control::NoBreakEnd
        //     | Control::RedactedBegin(_, _)
        //     | Control::RedactedEnd(_) => panic!("These Marker Controls shouldn't be passed to tui : {:#?}",item),
        //     Control::Str(_)
        //     |Control::Image(_, _, _)
        //     |Control::Bell(_)
        //     |Control::LF
        //     |Control::StrRedacted(_, _)
        //     |Control::Audio(_) => self.display.push(item),
        // }
    }
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
    pub articles: Vec<String>,
    pub list_state: ListState,
}
impl ListArticleState {
    pub fn checked_down(&mut self) {
        let mut next = 0;
        if let Some(current) = self.list_state.selected() {
            next = if current + 1 >= self.articles.len() {
                self.articles.len() - 1
            } else {
                current + 1
            };
        } else {
            next = 0;
        }
        self.list_state.select(Some(next));
    }
    pub fn checked_up(&mut self) {
        if let Some(current) = self.list_state.selected() {
            let next = if current < 1 { 0 } else { current - 1 };
            self.list_state.select(Some(next));
        } else {
            self.list_state.select(Some(0));
        }
    }
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
            ui: AppUIState::default(),
            articles: Helper::read_files_to_map(Helper::build_article_list(ARTICLE_DIR)),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        use image::io::Reader;
        use ratatui_image::picker::Picker;
        use std::io::Cursor;
        Self {
            running: true,
            counter: 0,
            state: AppState::default(),
            logo: {
                let data = Cursor::new(
                    Assets::get("static/SCP.png")
                        .expect("无法打开静态资源SCP.png")
                        .data,
                );
                let image = Reader::with_format(data, image::ImageFormat::Png);
                let dyn_img = image.decode().unwrap();
                let mut picker = Picker::from_termios(None).unwrap();
                
                picker.new_state(dyn_img) as _
            },
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
            AppUIState::Starting(x) => App::tick_starting(x, state),
            AppUIState::ListArticles(_x) => (),
            AppUIState::DisplayArticles(_) => todo!(),
        };
    }
    fn tick_starting(s: StartingState, state: &mut AppState) {
        let next_state = match s {
            StartingState::Finished => AppUIState::ListArticles(
                ListArticleState {
                    articles: state.articles.keys().cloned().collect(),
                    list_state: {
                        let mut new = ListState::default();
                        new.select(Some(0));
                        new
                    },
                }
                .into(),
            ),
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
    fn tick_list(x: usize) {
        if x > 0 {
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
    pub fn try_select_article(&mut self) {
        let NOT_FOUND_HTML: String = "<body>404 Not Found</body>".to_string();
        let new_state = match &self.state.ui {
            AppUIState::ListArticles(x) => {
                if let Some(index) = x.borrow().list_state.selected() {
                    let articles_borrow = &x.borrow().articles;
                    let article_path = articles_borrow.get(index).expect("选择的文章超出数组长度");
                    let article = self.state.articles.get(article_path).unwrap_or(&NOT_FOUND_HTML);
                    let new_state = AppUIState::DisplayArticles(ArticleState {
                        article: article.to_string(),
                        progress: 0.0,
                        mask_progress: 0.0,
                        crypt: HashMap::default(),
                        bookmark: 0,
                    });
                    Some(new_state)
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(new_state) = new_state {
            self.state.ui = new_state;
        }
    }
    pub fn article_up(&mut self) {
        let mut new_state;
        if let AppUIState::ListArticles(s) = &self.state.ui {
            new_state = s.clone();
            new_state.get_mut().checked_up();
            self.state.ui = AppUIState::ListArticles(new_state);
        }
    }
    pub fn article_down(&mut self) {
        let mut new_state;
        if let AppUIState::ListArticles(s) = &self.state.ui {
            new_state = s.clone();
            new_state.get_mut().checked_down();
            self.state.ui = AppUIState::ListArticles(new_state);
        }
    }

    // pub fn load_articles(&mut self) {
    //     
    //     let files = Self::build_article_list(ARTICLE_DIR);
    //     let map = Self::read_files_to_map(files);
    // }
}

struct Helper;
impl Helper {
    fn build_article_list(dir:&str) -> Vec<String> {
        let mut html_files = Vec::new();
        use walkdir::WalkDir;
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "html" {
                        if let Some(file_name) = entry.file_name().to_str() {
                            html_files.push(file_name.to_owned());
                        }
                    }
                }
            }
        }
        html_files
    }
    fn read_files_to_map(files: Vec<String>) -> BTreeMap<String, String> {
        let mut file_contents = BTreeMap::new();

        for file in files {
            if let Ok(content) = fs::read_to_string(file.clone()) {
                // if let Some(file_name) = file.strip_suffix(".html") {
                file_contents.insert(file.to_owned(), content);
                // }
            }
        }

        file_contents
    }
}


use ratatui::widgets::ListState;
use ratatui_image::protocol::ResizeProtocol;

use crate::assets::Assets;
