use eframe::egui::{Context, TextureHandle};
use egui::{ColorImage, TextureOptions};
use image::{DynamicImage, RgbaImage};
use std::fs;
use std::path::PathBuf;

// Sprites
pub const ADULT_GREEN_DRAGON: &str = "adult_green_dragon";
pub const BABY_GREEN_DRAGON: &str = "baby_green_dragon";
pub const YOUNG_RED_DRAGON: &str = "young_red_dragon";
pub const SLASH_ATTACK: &str = "slash_attack";

pub(crate) const SPRITE_DATA: [(&str, &[u8], u32, u32); 3] = [
    (ADULT_GREEN_DRAGON, include_bytes!("Basic Dragon Animations/Adult Green Dragon/AdultGreenDragon.png"), 16, 16),
    (BABY_GREEN_DRAGON, include_bytes!("Basic Dragon Animations/Baby Green Dragon/BabyGreenDragon.png"), 16, 16),
    (YOUNG_RED_DRAGON, include_bytes!("Basic Dragon Animations/Young Red Dragon/YoungRedDragon.png"), 16, 16),
];

pub(crate) const SPRITE_FOLDERS: [(&str, &str); 1] = [
    (SLASH_ATTACK, "src/ui/asset/sprite/attacks/slash")
];

#[derive(Clone)]
pub struct SpriteSheet {
    frames: Vec<TextureHandle>,
}

impl SpriteSheet {
    pub fn new(ctx: &Context, name: &str, bytes: &[u8], frame_width: u32, frame_height: u32) -> Self {
        let mut frames = Vec::new();

        // Load the full sprite sheet from memory
        let sprite_sheet = image::load_from_memory(bytes).expect("Failed to load sprite sheet");
        let sheet_width = sprite_sheet.width();
        let sheet_height = sprite_sheet.height();

        let frame_count_x = sheet_width / frame_width;
        let frame_count_y = sheet_height / frame_height;

        for y in 0..frame_count_y {
            for x in 0..frame_count_x {
                let frame = extract_frame(&sprite_sheet, x, y, frame_width, frame_height);
                let color_image = image_to_color_image(&frame);
                let texture = ctx.load_texture(
                    &format!("{}_frame_{}_{}", name, y, x),
                    color_image,
                    TextureOptions::default(),
                );
                frames.push(texture);
            }
        }

        Self { frames }
    }

    pub fn from_folder(ctx: &Context, name: &str, folder_path: &str) -> Self {
        let mut frames = Vec::new();

        let path = PathBuf::from(folder_path);

        println!("Path: {:?}", path);

        // Read directory and collect file paths
        let mut files: Vec<PathBuf> = fs::read_dir(&path)
            .expect("Failed to read sprite folder")
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().map_or(false, |ext| ext == "png"))
            .collect();

        // Sort files alphabetically to ensure frame order
        files.sort();
        println!("files sorted");

        // Load each image file
        for (index, file) in files.iter().enumerate() {
            println!("loading image {}", index);
            let image_data = fs::read(file).expect("Failed to read image file");
            let sprite = image::load_from_memory(&image_data).expect("Failed to load image");

            let color_image = image_to_color_image(&sprite.to_rgba8());
            let texture = ctx.load_texture(
                &format!("{}_frame_{}", name, index),
                color_image,
                TextureOptions::default(),
            );

            frames.push(texture);
        }

        Self { frames }
    }

    pub fn get_frame(&self, index: usize) -> &TextureHandle {
        &self.frames[index % self.frames.len()]
    }

    pub fn get_frame_count(&self) -> usize {
        self.frames.len()
    }
}

fn extract_frame(sheet: &DynamicImage, x: u32, y: u32, frame_width: u32, frame_height: u32) -> RgbaImage {
    let x_offset = x * frame_width;
    let y_offset = y * frame_height;
    sheet.crop_imm(x_offset, y_offset, frame_width, frame_height).to_rgba8()
}


fn image_to_color_image(img: &RgbaImage) -> ColorImage {
    let (width, height) = img.dimensions();
    let pixels = img.as_flat_samples();

    ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels.as_slice(),
    )
}