use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::SPRITE_SHEETS;
use crate::ui::asset::sprite::sprite_sheet::{SpriteSheet, SPRITE_DATA};
use eframe::egui;
use eframe::egui::ColorImage;
use eframe::epaint::TextureHandle;
use egui::Context;
use std::collections::HashMap;
use std::sync::Arc;

// Icons
pub const ADVENTURE_IMAGE_BYTES: &[u8] = include_bytes!("icon/adventure.png");
pub const SETTINGS_IMAGE_BYTES: &[u8] = include_bytes!("icon/settings.png");
pub const SHOP_IMAGE_BYTES: &[u8] = include_bytes!("icon/shop.png");
pub const UPGRADE_IMAGE_BYTES: &[u8] = include_bytes!("icon/upgrade.png");
pub const EXIT_IMAGE_BYTES: &[u8] = include_bytes!("icon/exit.png");


const ICON_DATA: [(&str, &[u8]); 5] = [
    ("adventure", ADVENTURE_IMAGE_BYTES),
    ("settings", SETTINGS_IMAGE_BYTES),
    ("shop", SHOP_IMAGE_BYTES),
    ("upgrade", UPGRADE_IMAGE_BYTES),
    ("exit", EXIT_IMAGE_BYTES),
];

pub fn load_icons(ctx: &Context) -> HashMap<String, TextureHandle> {
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

pub fn load_icons_inverted(ctx: &Context) -> HashMap<String, TextureHandle> {
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

pub fn load_sprite_sheets(ctx: &Context, game_data: &Arc<GameData>) {
    let mut sprite_sheets = HashMap::new();

    for (name, bytes) in SPRITE_DATA {
        let sprite_sheet = SpriteSheet::new(ctx, name, bytes, 16, 16);
        sprite_sheets.insert(name.to_string(), sprite_sheet);
    }

    game_data.set_field(SPRITE_SHEETS, sprite_sheets);
}