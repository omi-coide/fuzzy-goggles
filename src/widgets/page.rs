use std::{collections::HashMap, cell::RefCell, rc::Rc};

use ratatui::{
    widgets::StatefulWidget, prelude::Rect,
};
use html2text::Control;
use ansi_to_tui::IntoText;
use ratatui_image::{picker::Picker, ResizeImage, protocol::ResizeProtocol};
use unicode_width::UnicodeWidthChar;

use crate::assets::Assets;
pub struct PageDisplay {}
#[derive(Clone)]
pub struct PageState {
    pub progress:f32,
    pub rendered: Vec<Control>,
    pub height: u16,
    pub width: u16,
    pub to_draw: Vec<Control>,
    pub image_cache: Rc<RefCell<HashMap<String,Box<dyn ResizeProtocol>>>>,
    /// 跳过动画
    pub skip_draw: bool
    // pub acl: Rc<RefCell<HashMap<uuid::Uuid, bool>>>
}
// static mut cache: HashMap<String,Box<dyn ResizeProtocol>> = HashMap::default();
impl StatefulWidget for PageDisplay {
    type State = PageState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let mut x:u16=area.left();
        let mut y:u16=area.top();
        if area.right() > buf.area().right() || area.bottom() > buf.area().bottom(){
            return;
        }
        for item in &state.rendered {
            if x > area.right() || y>area.bottom() { return;}
            match item {
                Control::Str(s) => {
                    let text = s.into_text().unwrap();
                    for l in text {
                        for s in l.spans {
                            for c in s.content.chars() {
                                if x >= buf.area().right() || y>=buf.area().bottom() { return;}
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
                    if x >= buf.area().right() || y>=buf.area().bottom() { return;}
                    if x+ *w as u16 > area.right() || y+*h as u16 >area.bottom() { return;}
                    image_src.render(Rect{x,y,width: *w as u16,height:*h as u16}, buf,outerimage);
                    y+= *h as u16;
                    x=area.left();
                },
                Control::Bell(_) => todo!(),
                Control::LF => {
                    x = area.left();
                    y +=1;
                },
                Control::StrRedacted(_, _) => {
                    // let decrypted = state.

                },
                Control::Audio(_) => todo!(),
                _ => unreachable!(),
            }
        }
        if !state.to_draw.is_empty() {
            let mut first = state.to_draw.first_mut().unwrap();
            match first {
                Control::Str(ref mut s)
                |Control::StrRedacted(ref mut s, _)  => {
                    if s.is_empty() {
                        state.to_draw.remove(0);
                        return;
                    }
                    if let Some(Control::Str(last)) =  state.rendered.last_mut(){
                        last.push(s.remove(0));
                    } else {
                        state.rendered.push(Control::Str(s.remove(0).to_string()));
                    }
                },
                Control::Image(_, _, _) => {
                    state.rendered.push(state.to_draw.remove(0));
                },
                Control::Bell(_) => todo!(),
                Control::LF => {
                    state.rendered.push(state.to_draw.remove(0));
                },
                Control::Audio(_) => {
                    todo!("完成声音副作用"); 
                    state.rendered.push(state.to_draw.remove(0));

                },
                _ => unreachable!()
            }
        }
        
    }
}