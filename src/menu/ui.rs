use std::sync::LazyLock;

use yakui::font::{
	Font, 
	FontName, 
	FontSettings, 
	Fonts
};

pub const FONT: LazyLock<FontName> = LazyLock::new(|| { FontName::new("PixeloidMono") });

pub fn init_ui() {
	yakui_macroquad::cfg(|cfg| {
		let fonts = cfg.dom().get_global_or_init(Fonts::default);
		fonts.add(
			Font::from_bytes(
				include_bytes!("../../assets/fonts/PixeloidMono.ttf").as_slice(), 
				FontSettings::default()
			).unwrap(), 
			Some("PixeloidMono")
		);
	});
}
