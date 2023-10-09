use std::io::Cursor;

use ratatui::widgets::Widget;
use rust_embed::RustEmbed;
use image::io::Reader;
use ratatui_image::{
    picker::{Picker, ProtocolType},
    Resize, ResizeImage, protocol::{ImageSource, ResizeProtocol},
  };

use crate::assets::Assets;
struct Logo{
    image: image::DynamicImage
}
// impl Default for Logo {
//     fn default() -> Self {
//         let data = Cursor::new(Assets::get("static/SCP.png").unwrap());
//         let image = Reader::with_format(data,image::ImageFormat::Png);
//         Self {  image: image.decode().unwrap() }
//     }
// }
impl Widget for Logo {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut picker = Picker::from_termios(None).unwrap();
        picker.set(ProtocolType::Halfblocks);
        let image = ResizeImage::new(None);
        //ra::render();
    }
}