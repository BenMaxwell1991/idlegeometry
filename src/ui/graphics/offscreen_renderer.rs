use crate::ui::asset::loader::load_sprites_native;
use crate::ui::asset::sprite::sprite_sheet::SpriteSheet;
use crate::ui::graphics::gl::{create_rect_shader_program, create_sprite_shader_program};
use glow::*;
use rustc_hash::FxHashMap;
use std::sync::Arc;

pub struct OffscreenRenderer {
    gl: Arc<Context>,
    framebuffer: Framebuffer,
    texture: Texture,
    width: i32,
    height: i32,
    pub rect_shader: NativeProgram,
    pub sprite_shader: NativeProgram,
    pub sprite_sheets: FxHashMap<String, SpriteSheet>,
}

impl OffscreenRenderer {
    pub fn new(gl: Arc<Context>, width: i32, height: i32) -> Self {
        unsafe {
            let framebuffer = gl.create_framebuffer().unwrap();
            let texture = gl.create_texture().unwrap();
            let rect_shader = create_rect_shader_program(&gl);
            let sprite_shader = create_sprite_shader_program(&gl);

            gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
            gl.enable(BLEND);
            gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);

            // Create the texture to render into
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.tex_image_2d(
                TEXTURE_2D,
                0,
                RGBA as i32,
                width,
                height,
                0,
                RGBA,
                UNSIGNED_BYTE,
                PixelUnpackData::Slice(None),
            );
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

            // Attach texture to framebuffer
            gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(texture), 0);

            assert_eq!(gl.check_framebuffer_status(FRAMEBUFFER), FRAMEBUFFER_COMPLETE);

            gl.bind_framebuffer(FRAMEBUFFER, None); // Unbind

            let sprite_sheets = load_sprites_native(&gl);

            Self {
                gl,
                framebuffer,
                texture,
                width,
                height,
                rect_shader,
                sprite_shader,
                sprite_sheets,
            }
        }
    }
    pub fn resize(&mut self, new_width: i32, new_height: i32) {
        if self.width == new_width && self.height == new_height {
            return;
        }

        println!("Resizing framebuffer to: {}x{}", new_width, new_height);

        let gl = &self.gl;
        unsafe {
            // Delete old framebuffer and texture
            gl.delete_framebuffer(self.framebuffer);
            gl.delete_texture(self.texture);

            // Create new framebuffer and texture
            let framebuffer = gl.create_framebuffer().unwrap();
            let texture = gl.create_texture().unwrap();

            gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));

            // Create the resized texture to render into
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.tex_image_2d(
                TEXTURE_2D,
                0,
                RGBA as i32,
                new_width,
                new_height,
                0,
                RGBA,
                UNSIGNED_BYTE,
                PixelUnpackData::Slice(None),
            );
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

            // Attach texture to framebuffer
            gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(texture), 0);

            assert_eq!(gl.check_framebuffer_status(FRAMEBUFFER), FRAMEBUFFER_COMPLETE);

            gl.bind_framebuffer(FRAMEBUFFER, None); // Unbind

            // Update stored values
            self.framebuffer = framebuffer;
            self.texture = texture;
            self.width = new_width;
            self.height = new_height;
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.framebuffer));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
        }
    }

    pub fn get_texture(&self) -> Texture {
        self.texture
    }

    pub fn get_gl(&self) -> Arc<Context> {
        self.gl.clone()
    }
}
