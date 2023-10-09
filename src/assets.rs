use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "static/"]
#[prefix = "static/"]
pub struct Assets;