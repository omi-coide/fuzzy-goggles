

use ratatui::widgets::Widget;


use ratatui_image::{
    picker::{Picker, ProtocolType}, ResizeImage,
  };


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
    fn render(self, _area: ratatui::prelude::Rect, _buf: &mut ratatui::prelude::Buffer) {
        let mut picker = Picker::from_termios(None).unwrap();
        picker.set(ProtocolType::Halfblocks);
        let _image = ResizeImage::new(None);
        //ra::render();
    }
}