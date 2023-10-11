use std::{collections::HashMap, hash::Hash, cell::RefCell, rc::Rc};

use ratatui::{
    widgets::StatefulWidget, prelude::Rect,
};
use html2text::Control;
use ansi_to_tui::IntoText;
use ratatui_image::{picker::Picker, ResizeImage, protocol::ResizeProtocol};
use unicode_width::UnicodeWidthChar;

use crate::assets::Assets;
pub struct PageDisplay {}
pub struct PageState {
    pub progress:f32,
    pub rendered: Vec<Control>,
    pub to_draw: Vec<Control>,
    pub image_cache: Rc<RefCell<HashMap<String,Box<dyn ResizeProtocol>>>>
}
// static mut cache: HashMap<String,Box<dyn ResizeProtocol>> = HashMap::default();
impl StatefulWidget for PageDisplay {
    type State = PageState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let mut x:u16=area.left();
        let mut y:u16=area.top();
        for item in &state.rendered {
            match item {
                Control::Str(s) => {
                    let text = s.into_text().unwrap();
                    for l in text {
                        for s in l.spans {
                            for c in s.content.chars() {
                                buf.get_mut(x, y).set_style(s.style);
                                buf.get_mut(x, y).set_char(c);
                                x+=c.width().unwrap_or(0) as u16;
                            }
                        }
                    }
                },
                Control::Image(src, w, h) => {
                    let mut outerimage;
                    let mut cache = state.image_cache.borrow_mut();
                    if cache.contains_key(src) {
                        outerimage = cache.get_mut(src).unwrap();
                    } else {
                        let data = std::io::Cursor::new(
                            Assets::get(src)
                                .expect(&format!("无法打开静态资源{src}"))
                                .data,
                        );
                        let image = image::io::Reader::with_format(data, image::ImageFormat::Png);
                        let dyn_img = image.decode().unwrap();
                        let mut picker = Picker::from_termios(None).unwrap();
                        let mut aimage = picker.new_state(dyn_img);
                        cache.insert(src.to_string(), aimage);
                        outerimage = cache.get_mut(src).unwrap();
                    }
                    let image_src = ResizeImage::new(None);
                    image_src.render(Rect{x,y,width: *w as u16,height:*h as u16}, buf,outerimage);
                    y+= *h as u16;
                    x=area.left();
                },
                Control::Bell(_) => todo!(),
                Control::LF => {
                    x = area.left();
                    y +=1;
                },
                Control::StrRedacted(_, _) => todo!(),
                Control::Audio(_) => todo!(),
                _ => unreachable!(),
            }
        }
        
    }
}