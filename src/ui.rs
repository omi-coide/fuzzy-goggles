use std::collections::HashMap;

use crate::app::{App, StartingState, Page};


use html2text::{just_render, just_parse ,Control};
use ratatui::prelude::{Constraint, Direction, Frame, Layout};

use ratatui::{
    backend::Backend,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
};




use ratatui::widgets::List;
use ratatui_image::ResizeImage;
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
            let _margin = (frame.size().width - height)/2;
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
        crate::app::AppUIState::DisplayArticles(s) => {
            // frame.render_widget()
            let s = s.get_mut();
            use stringreader::StringReader;
            let layout = Layout::default().direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(3),
                Constraint::Length(3),   //Status bar and text input
                ])
                .split(frame.size());
            let mut skip_rerender = false;
            if s.page_state.height == layout[0].height.into() && s.page_state.width == layout[0].width.into() {
                skip_rerender = true;
            }
            if !skip_rerender {
                s.page_state.height = layout[0].height.into();
                s.page_state.width = layout[0].width.into();
                let reader = StringReader::new(s.article.as_str());
                let render_tree = just_parse(reader);
                let mut controls = just_render(render_tree, layout[0].width.into(), my_map).expect("文本解析错误");
                
                let pages: Vec<Page> =try_build_page(&mut controls,layout[0].height.into());
                s.pages = pages;
                let pageindex = (s.pages.len()-1)* s.index as usize;
                let page = &s.pages[pageindex];
                s.page_state.rendered.clear();
                s.page_state.to_draw = page.lines.clone();
            }
            if s.pages.is_empty() {
                frame.render_widget(Paragraph::new("无内容"), layout[0]);
                frame.render_widget(Paragraph::new(format!("页{}/{}",0,s.pages.len())), layout[1]);
                return;
            }
            let pageindex = (s.pages.len()-1)* s.index as usize;
            let mut pagestate = &mut s.page_state;
            frame.render_widget(Paragraph::new(format!("页{}/{}, 宽{}高{}",pageindex+1,s.pages.len(), layout[0].width,layout[0].height)), layout[1]);
            frame.render_stateful_widget(crate::widgets::page::PageDisplay{}, layout[0], &mut pagestate);
        },
    }
}

use html2text::try_build_block;

fn try_build_page(controls:&mut Vec<Control>,max_height:u16)->Vec<Page>{
    let mut height:usize = 0;
    let mut page = Page::default();
    let mut result = vec![];
    let blocks = try_build_block(controls);
    for mut block in blocks {
        if height + block.height > max_height.into() {
            result.push(page);
            page = Page::default();
            page.lines = block.inner;
            height = block.height;
        } else {
            page.lines.append(&mut block.inner);
            height += block.height;
        }
    }
    if !page.lines.is_empty() {
        result.push(page);
    }
    result
}
use html2text::render::text_renderer::RichAnnotation;

pub fn my_map(
    annotation: &RichAnnotation,
) -> (String, Box<dyn Fn(&String) -> String>, String) {
    use termion::color::*;
    use RichAnnotation::*;
    match annotation {
        Default => ("".into(), Box::new(|s| s.to_string()), "".into()),
        Link(_) => (
            format!("{}", termion::style::Underline),
            Box::new(|s| s.to_string()),
            format!("{}", termion::style::Reset),
        ),
        Image(_,..) => (
            format!("{}", Fg(Blue)),
            Box::new(|s| s.to_string()),
            format!("{}", Fg(Reset)),
        ),
        Emphasis => (
            format!("{}", termion::style::Bold),
            Box::new(|s| s.to_string()),
            format!("{}", termion::style::Reset),
        ),
        Strong => (
            format!("{}", Fg(LightYellow)),
            Box::new(|s| s.to_string()),
            format!("{}", Fg(Reset)),
        ),
        Strikeout => (
            format!("{}", Fg(LightBlack)),
            Box::new(|s| s.to_string()),
            format!("{}", Fg(Reset)),
        ),
        Code => (
            format!("{}", Fg(Blue)),
            Box::new(|s| s.to_string()),
            format!("{}", Fg(Reset)),
        ),
        Preformat(_) => (
            format!("{}", Fg(Blue)),
            Box::new(|s| s.to_string()),
            format!("{}", Fg(Reset)),
        ),
        Colored(c) => (
            (format!(
                "{}",
                Fg(AnsiValue(colvert::ansi256_from_rgb((c.r, c.g, c.b))))
            )),
            Box::new(|s| s.to_string()),
            format!("{}", Fg(Reset)),
        ),
        Bell => todo!(),
        NoBreakBegin => (String::new(), Box::new(|s| s.to_string()), String::new()),
        NoBreakEnd => (String::new(), Box::new(|s| s.to_string()), String::new()),
        RedactedBegin(_, _) => (String::new(), Box::new(|s| s.to_string()), String::new()),
        RedactedEnd(_, _) => (String::new(), Box::new(|s| s.to_string()), String::new()),
        Custom(_, _) => (String::new(), Box::new(|s| s.to_string()), String::new()),
    }
}


