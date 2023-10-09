use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap, Widget, StatefulWidget},
    Frame, Terminal, style::Style,
};
struct Mask {}
struct MaskState {
    pub progress:f32,
}
impl StatefulWidget for Mask {
    type State = MaskState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // area should always be full frame
        let start = f32::floor(state.progress * area.width as f32 * area.height as f32) as usize;
        let mut start_x = 0;
        let mut start_y = 0;
        let width = area.width;
        let height = area.height;
        while (start_y*width + start_x) as usize <= start {
            if start_x + 1 >= width {
                start_x = 0;
                start_y += 1;
            } else {
                start_x += 1;
            }
            if start_y +1 == height && start_x + 1 >= width {
                break;
            }
        }
        // try_skip
        for i in start_x..width {
            if buf.get(i, start_y).symbol.eq(" "){
                state.progress += 1.0 / (area.width as f32 * area.height as f32) as f32;
            }
        }
        let len = buf.content.len();
        for index in 0..len {
            let should_clear:bool;
            let (x,y) = buf.pos_of(index);
            should_clear = !is_in(x,y,start_x,start_y);
            if should_clear {
                if buf.get(x, y).symbol.eq(" "){
                    continue;
                } else {
                    buf.get_mut(x, y).set_char('*');
                }  
            }
        }
        fn is_in(x:u16,y:u16,x_:u16,y_:u16)-> bool {
            // if this is true, should NOT clear;
            if y < y_ {
                return true;
            } else if y==y_{
                return x<=x_;
            } else {
                return false;
            }
        }
    }
}