use crate::app::{App, StartingState};
use crate::assets::Assets;
use once_cell::sync::Lazy;
use ratatui::prelude::{Constraint, Direction, Frame, Layout};
use ratatui::style::Modifier;
use ratatui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
};
use std::cell::{Cell, RefCell};
use std::io::Cursor;

use image::io::Reader;
use ratatui::widgets::{List, Widget};
use ratatui_image::{
    picker::{Picker, ProtocolType},
    protocol::{ImageSource, ResizeProtocol},
    Resize, ResizeImage,
};
use rust_embed::RustEmbed;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match &mut app.state.ui {
        crate::app::AppUIState::Starting(x) => {
            let StartingState::InProgress(x) = x else {
                return;
            };
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Min(5),
                    Constraint::Min(15),
                    Constraint::Length(3),
                ])
                .split(frame.size());
            let rect_for_logo = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25),Constraint::Percentage(50),Constraint::Percentage(25)])
            .split(layout[0]);
            let image_widget = ResizeImage::new(None);
            frame.render_stateful_widget(image_widget, rect_for_logo[1], &mut app.logo);
            frame.render_widget(
                Paragraph::new(format!(
                    "This is a tui template.\n\
                    Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                    Press left and right to increment and decrement the counter respectively.\n\
                    Counter: {}",
                    app.counter
                )),
                layout[1],
            );
            frame.render_widget(
                Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Loading"))
                    .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
                    .percent(*x as u16),
                layout[2],
            );
        }
        crate::app::AppUIState::ListArticles(ref mut l) => {
            let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(20),
                Constraint::Min(15),
                Constraint::Length(3),
            ])
            .split(frame.size());
            use ratatui::widgets::ListItem;
            let list = List::new(
                l.borrow()
                    .articles
                    .iter()
                    .map(|a| ListItem::new(a.to_string()))
                    .collect::<Vec<_>>(),
            )
            .block(Block::default().borders(Borders::ALL).title("Select Record..."))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
            frame.render_stateful_widget(list, frame.size(), &mut l.get_mut().list_state)
        }
        crate::app::AppUIState::DisplayArticles(_) => {
            frame.render_widget()
        },
    }
}



