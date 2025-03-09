use eframe::egui::{Context, TextureHandle};
use egui::{ColorImage, TextureOptions};
use image::{DynamicImage, RgbaImage};

// Sprites
pub const ADULT_GREEN_DRAGON: &str = "adult_green_dragon";
pub const BABY_GREEN_DRAGON: &str = "baby_green_dragon";

pub(crate) const SPRITE_DATA: [(&str, &[u8]); 2] = [
    (ADULT_GREEN_DRAGON, include_bytes!("Basic Dragon Animations/Adult Green Dragon/AdultGreenDragon.png")),
    (BABY_GREEN_DRAGON, include_bytes!("Basic Dragon Animations/Baby Green Dragon/BabyGreenDragon.png")),
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
        let frame_count = sheet_width / frame_width;

        for i in 0..frame_count {
            let frame = extract_frame(&sprite_sheet, i, frame_width, frame_height);
            let color_image = image_to_color_image(&frame);
            let texture = ctx.load_texture(&format!("{}_frame_{}", name, i), color_image, TextureOptions::default());
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

fn extract_frame(sheet: &DynamicImage, index: u32, frame_width: u32, frame_height: u32) -> RgbaImage {
    let x_offset = index * frame_width;
    sheet.crop_imm(x_offset, 0, frame_width, frame_height).to_rgba8()
}

fn image_to_color_image(img: &RgbaImage) -> ColorImage {
    let (width, height) = img.dimensions();
    let pixels = img.as_flat_samples();

    ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels.as_slice(),
    )
}