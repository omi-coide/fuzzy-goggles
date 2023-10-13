#![feature(const_trait_impl)]
use yly_tui::app::{App, AppResult};
use yly_tui::event::{Event, EventHandler};
use yly_tui::handler::{handle_key_events, handle_resize};
use yly_tui::tui::Tui;
use std::borrow::BorrowMut;
use std::time::Duration;
use std::{io, thread};
use std::sync::{Mutex, RwLock, Arc};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

// pub static FONT_SIZE : Arc<RwLock<(u16,u16)>> = Arc::new(RwLock::new((0,0)));
fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();
    // let app_lock = Arc::new(Mutex::new(Box::new(app)));
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;
    let events = EventHandler::new(50);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    const WIDTH:u16 = 80;
    const HEIGHT:u16 = 40;


    // Start the main loop.
    // ui 线程
    // thread::spawn(move ||{
    //     let app_ui = app_lock.clone();
    //     loop {
    //         let mut locked = app_ui.lock().unwrap();
    //         // Render the user interface.
    //         tui.draw(&mut locked);
    //         drop(locked);
    //         thread::sleep(Duration::from_millis(60));
    //     }
    // });
    // 事件线程
    while app.running {
        tui.draw(&mut app);

        match tui.events.next()? {
            Event::Tick => App::tick(&mut app.state),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {},
            Event::Resize(x, y) => handle_resize(x,y, &mut app)?,
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
