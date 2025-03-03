use eframe::egui;
use eframe::egui::ColorImage;
use eframe::epaint::TextureHandle;
use std::collections::HashMap;

pub const GEOMETRY_IMAGE_BYTES: &[u8] = include_bytes!("icon/geometry.png");
pub const SETTINGS_IMAGE_BYTES: &[u8] = include_bytes!("icon/settings.png");
pub const SHOP_IMAGE_BYTES: &[u8] = include_bytes!("icon/shop.png");
pub const UPGRADE_IMAGE_BYTES: &[u8] = include_bytes!("icon/upgrade.png");
pub const EXIT_IMAGE_BYTES: &[u8] = include_bytes!("icon/exit.png");

const ICON_DATA: [(&str, &[u8]); 5] = [
    ("geometry", GEOMETRY_IMAGE_BYTES),
    ("settings", SETTINGS_IMAGE_BYTES),
    ("shop", SHOP_IMAGE_BYTES),
    ("upgrade", UPGRADE_IMAGE_BYTES),
    ("exit", EXIT_IMAGE_BYTES),
];

pub fn load_icons(ctx: &egui::Context) -> HashMap<String, TextureHandle> {
    let mut icons = HashMap::new();

    for (name, bytes) in ICON_DATA {
        if let Ok(image) = image::load_from_memory(bytes) {
            let rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();
            let color_image = ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                rgba.as_raw(),
            );

            let texture = ctx.load_texture(name, color_image, Default::default());
            icons.insert(name.to_string(), texture);
        }
    }

    icons
}

pub fn load_icons_inverted(ctx: &egui::Context) -> HashMap<String, TextureHandle> {
    let mut icons = HashMap::new();

    for (name, bytes) in ICON_DATA {
        if let Ok(image) = image::load_from_memory(bytes) {
            let mut rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();

            for pixel in rgba.chunks_exact_mut(4) {
                pixel[0] = 255 - pixel[0];
                pixel[1] = 255 - pixel[1];
                pixel[2] = 255 - pixel[2];
            }

            let color_image = ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                rgba.as_raw(),
            );

            let texture = ctx.load_texture(name, color_image, Default::default());
            icons.insert(name.to_string(), texture);
        }
    }

    icons
}