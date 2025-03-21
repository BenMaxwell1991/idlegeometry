use crate::game::data::game_data::GameData;
use crate::ui::asset::sprite::sprite_sheet::{SpriteSheet, SPRITE_DATA, SPRITE_FOLDERS};
use eframe::egui;
use eframe::egui::ColorImage;
use egui::{Context, FontData, FontDefinitions, FontFamily};
use rustc_hash::FxHashMap;
use std::sync::Arc;


// Icons
pub const ADVENTURE_IMAGE_BYTES: &[u8] = include_bytes!("icon/adventure.png");
pub const SETTINGS_IMAGE_BYTES: &[u8] = include_bytes!("icon/settings.png");
pub const SHOP_IMAGE_BYTES: &[u8] = include_bytes!("icon/shop.png");
pub const UPGRADE_IMAGE_BYTES: &[u8] = include_bytes!("icon/upgrade.png");
pub const EXIT_IMAGE_BYTES: &[u8] = include_bytes!("icon/exit.png");
pub const COIN_IMAGE_BYTES: &[u8] = include_bytes!("icon/icon_coin.png");
pub const RUBY_IMAGE_BYTES: &[u8] = include_bytes!("icon/icon_ruby.png");

// Fonts
pub const SUPER_SHINY_FONT: &str = "super_shiny";
pub const SUPER_SHINY_FONT_BYTES: &[u8] = include_bytes!("font/super_shiny.ttf");


const ICON_DATA: [(&str, &[u8]); 7] = [
    ("adventure", ADVENTURE_IMAGE_BYTES),
    ("settings", SETTINGS_IMAGE_BYTES),
    ("shop", SHOP_IMAGE_BYTES),
    ("upgrade", UPGRADE_IMAGE_BYTES),
    ("exit", EXIT_IMAGE_BYTES),
    ("coin", COIN_IMAGE_BYTES),
    ("ruby", RUBY_IMAGE_BYTES),
];

pub fn load_icons(ctx: &Context, game_data: &GameData) {
    let mut icons = FxHashMap::default();

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

    *game_data.icons.write().unwrap() = icons;
}

pub fn load_icons_inverted(ctx: &Context, game_data: &GameData) {
    let mut icons = FxHashMap::default();

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

    *game_data.icons_inverted.write().unwrap() = icons;
}

pub fn register_custom_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        SUPER_SHINY_FONT.to_owned(),
        Arc::from(FontData::from_static(SUPER_SHINY_FONT_BYTES)),
    );

    fonts.families
        .entry(FontFamily::Name(SUPER_SHINY_FONT.into()))
        .or_default()
        .push(SUPER_SHINY_FONT.to_owned());

    ctx.set_fonts(fonts);
}


pub fn load_sprites_native(gl: &glow::Context) -> FxHashMap<String, SpriteSheet> {
    let mut native_sprite_sheets = FxHashMap::default();

    load_sprite_sheets_native(gl, &mut native_sprite_sheets);

    load_sprite_folders_native(gl, &mut native_sprite_sheets);

    println!("✅ All native sprites loaded!");
    native_sprite_sheets
}

fn load_sprite_sheets_native(gl: &glow::Context, native_sprite_sheets: &mut FxHashMap<String, SpriteSheet>) {
    let total_sheets = SPRITE_DATA.len();

    for (index, (name, bytes, width, height)) in SPRITE_DATA.iter().enumerate() {
        println!(
            "[{}/{}] Loading native sprite sheet: '{}' ({:.1}% done)",
            index + 1, total_sheets, name, (index as f32 / total_sheets as f32) * 100.0
        );

        let sprite_sheet = SpriteSheet::new(gl, bytes, *width, *height);
        native_sprite_sheets.insert(name.to_string(), sprite_sheet);
    }
}

fn load_sprite_folders_native(gl: &glow::Context, native_sprite_sheets: &mut FxHashMap<String, SpriteSheet>) {
    let total_folders = SPRITE_FOLDERS.len();

    for (index, (name, folder_path)) in SPRITE_FOLDERS.iter().enumerate() {
        println!(
            "[{}/{}] Loading sprite folder: '{}' ({:.1}% done)",
            index + 1, total_folders, name, (index as f32 / total_folders as f32) * 100.0
        );

        let sprite_sheet = SpriteSheet::from_folder(gl, folder_path);
        native_sprite_sheets.insert(name.to_string(), sprite_sheet);
    }
}