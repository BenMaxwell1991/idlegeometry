use egui::ColorImage;
use glow::{HasContext, NativeTexture, PixelUnpackData};
use image::{DynamicImage, RgbaImage};
use std::fs;
use std::fs::{create_dir_all, read, read_dir};
use std::path::PathBuf;

// Sprites
pub const ADULT_GREEN_DRAGON: &str = "adult_green_dragon";
pub const BABY_GREEN_DRAGON: &str = "baby_green_dragon";
pub const YOUNG_RED_DRAGON: &str = "young_red_dragon";
pub const AQUA_DRAKE: &str = "aqua_drake";
pub const ADULT_WHITE_DRAGON: &str = "adult_white_dragon";
pub const TREASURE: &str = "treasure";
pub const SLASH_ATTACK: &str = "slash_attack";

pub(crate) const SPRITE_DATA: [(&str, &[u8], u32, u32); 6] = [
    (ADULT_GREEN_DRAGON, include_bytes!("dragons/Basic Dragon Animations/Adult Green Dragon/AdultGreenDragon.png"), 16, 16),
    (BABY_GREEN_DRAGON, include_bytes!("dragons/Basic Dragon Animations/Baby Green Dragon/BabyGreenDragon.png"), 16, 16),
    (YOUNG_RED_DRAGON, include_bytes!("dragons/Basic Dragon Animations/Young Red Dragon/YoungRedDragon.png"), 16, 16),
    (AQUA_DRAKE, include_bytes!("dragons/Basic Dragon Animations/Aqua Drake/AquaDrake.png"), 16, 16),
    (ADULT_WHITE_DRAGON, include_bytes!("dragons/Basic Dragon Animations/Adult White Dragon/AdultWhiteDragon.png"), 16, 16),
    (TREASURE, include_bytes!("resources/treasure_sheet.png"), 16, 16),
];

pub(crate) const SPRITE_FOLDERS: [(&str, &str); 1] = [
    (SLASH_ATTACK, "src/ui/asset/sprite/attacks/slash")
];

#[derive(Clone)]
pub struct SpriteSheet {
    pub(crate) frame_texture_ids: Vec<NativeTexture>,
}

impl SpriteSheet {
    pub fn new(gl: &glow::Context, bytes: &[u8], frame_width: u32, frame_height: u32) -> Self {
        let mut frame_texture_ids = Vec::new();

        // Load the full sprite sheet from memory
        let sprite_sheet = image::load_from_memory(bytes).expect("Failed to load sprite sheet");
        let sheet_width = sprite_sheet.width();
        let sheet_height = sprite_sheet.height();

        let frame_count_x = sheet_width / frame_width;
        let frame_count_y = sheet_height / frame_height;

        for y in 0..frame_count_y {
            for x in 0..frame_count_x {
                let frame = extract_frame(&sprite_sheet, x, y, frame_width, frame_height);
                let native_texture = create_opengl_texture(gl, &frame);
                frame_texture_ids.push(native_texture);
            }
        }

        Self { frame_texture_ids }
    }

    pub fn from_folder(gl: &glow::Context, folder_path: &str) -> Self {
        let mut frame_texture_ids = Vec::new();

        let path = PathBuf::from(folder_path);

        println!("Path: {:?}", path);

        let mut files: Vec<PathBuf> = read_dir(&path)
            .expect("Failed to read sprite folder")
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().map_or(false, |ext| ext == "png"))
            .collect();

        files.sort();
        println!("✅ Files sorted.");

        for (index, file) in files.iter().enumerate() {
            println!("📂 Loading image {}", index);
            let image_data = fs::read(file).expect("Failed to read image file");
            let sprite = image::load_from_memory(&image_data).expect("Failed to load image");

            let rgba_image = sprite.to_rgba8();
            let pixels = rgba_image.as_raw();

            let native_texture = create_opengl_texture(gl, &sprite.to_rgba8());
            frame_texture_ids.push(native_texture);
        }

        Self { frame_texture_ids }
    }

    pub fn get_frame_native(&self, index: usize) -> NativeTexture {
        self.frame_texture_ids[index % self.frame_texture_ids.len()]
    }

    pub fn get_frame_count_native(&self) -> usize {
        self.frame_texture_ids.len()
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

fn create_opengl_texture(gl: &glow::Context, image: &RgbaImage) -> NativeTexture {
    let (width, height) = image.dimensions();
    let pixels = image.as_raw(); // Get raw pixel data

    unsafe {
        let texture = gl.create_texture().expect("Failed to create OpenGL texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::SRGB8_ALPHA8 as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            PixelUnpackData::Slice(Some(pixels))
        );

        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);

        gl.bind_texture(glow::TEXTURE_2D, None);
        texture
    }
}

pub fn convert_transparent_white_to_black(gl: &glow::Context, folder_path: &str) {
    let path = PathBuf::from(folder_path);
    println!("Path: {:?}", path);

    let mut files: Vec<PathBuf> = read_dir(&path)
        .expect("Failed to read sprite folder")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().map_or(false, |ext| ext == "png"))
        .collect();

    files.sort();
    println!("✅ Files sorted.");

    let output_folder = path.join("modified");
    create_dir_all(&output_folder).expect("Failed to create modified image folder");

    // Load and modify each image
    for (index, file) in files.iter().enumerate() {
        println!("📂 Loading image {}", index);
        let image_data = read(file).expect("Failed to read image file");
        let sprite = image::load_from_memory(&image_data).expect("Failed to load image");

        let mut rgba_image = sprite.to_rgba8();
        let pixels = rgba_image.as_mut();

        for chunk in pixels.chunks_exact_mut(4) {
            if chunk[0] == 255 && chunk[1] == 255 && chunk[2] == 255 && chunk[3] == 0 {
                chunk[0] = 0;
                chunk[1] = 0;
                chunk[2] = 0;
            }
        }

        let modified_file_path = output_folder.join(
            file.file_stem().unwrap().to_str().unwrap().to_owned() + "_mod.png"
        );

        rgba_image
            .save(&modified_file_path)
            .expect("Failed to save modified image");

        println!("✅ Saved modified image: {}", modified_file_path.display());
    }
}