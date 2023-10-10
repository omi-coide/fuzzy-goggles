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
            let height = 2*frame.size().height/3;
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Min(height),
                    Constraint::Min(4),
                    Constraint::Length(3),
                ])
                .split(frame.size());
            let margin = (frame.size().width - height)/2;
            let rect_for_logo = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(33),Constraint::Percentage(33),Constraint::Percentage(33)])
            .split(layout[0]);
            let image_widget = ResizeImage::new(None);
            frame.render_stateful_widget(image_widget, rect_for_logo[1], &mut app.logo);
            frame.render_widget(
                Paragraph::new(format!(
                    "This is a tui template.\n\
                    Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                    Press left and right to increment and decrement the counter respectively.\n\
                    Counter: {},image_width:{},height:{},terminal_width:{},height:{}",
                    app.counter,rect_for_logo[1].width,rect_for_logo[1].height,frame.size().width,frame.size().height
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
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(frame.size());
            let vlayout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(layout[1]);
            use ratatui::widgets::ListItem;
            let list = List::new(
                l.borrow()
                    .articles
                    .iter()
                    .map(|a| {
                        let item = if let Some(file_name) = a.strip_suffix(".html") {
                            file_name.to_string()
                        } else {
                            a.to_string()
                        };
                        ListItem::new(item)
                    })
                    .collect::<Vec<_>>(),
            )
            .block(Block::default().borders(Borders::ALL).title("Select Record..."))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::Green))
            .highlight_symbol(">>");
            frame.render_stateful_widget(list, vlayout[1], &mut l.get_mut().list_state)
        }
        crate::app::AppUIState::DisplayArticles(_) => {
            // frame.render_widget()
            todo!()
        },
    }
}



