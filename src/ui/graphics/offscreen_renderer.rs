use glow::*;
use std::sync::Arc;

pub struct OffscreenRenderer {
    gl: Arc<Context>,
    framebuffer: Framebuffer,
    texture: Texture,
    width: i32,
    height: i32,
}

impl OffscreenRenderer {
    pub fn new(gl: Arc<Context>, width: i32, height: i32) -> Self {
        unsafe {
            let framebuffer = gl.create_framebuffer().unwrap();
            let texture = gl.create_texture().unwrap();

            gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));

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

            Self {
                gl,
                framebuffer,
                texture,
                width,
                height,
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
