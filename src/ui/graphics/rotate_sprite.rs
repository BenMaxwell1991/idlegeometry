use egui::{Pos2, Vec2};
use glow::*;
use std::num::NonZeroU32;

pub struct RotatedSprite {
    pub texture_id: u32,
    pub position: Pos2,
    pub size: Vec2,
    pub rotation: f32,
    pub colour: [f32; 4],
}

pub fn draw_rotated_sprite(
    gl: &Context,
    sprite: &RotatedSprite,
    shader: &Option<NativeProgram>,
) {
    unsafe {
        if let Some(program) = *shader {
            gl.use_program(Some(program));

            // Convert rotation to radians
            let radians = sprite.rotation.to_radians();
            let cos_theta = radians.cos();
            let sin_theta = radians.sin();

            // Define the rotation matrix
            let rotation_matrix: [f32; 4] = [
                cos_theta, -sin_theta,
                sin_theta, cos_theta,
            ];

            // Send rotation matrix to the shader
            let rotation_location = gl.get_uniform_location(program, "rotationMatrix").unwrap();
            gl.uniform_2_f32_slice(Some(&rotation_location), &rotation_matrix);

            // Set position offset
            let position_location = gl.get_uniform_location(program, "positionOffset").unwrap();
            gl.uniform_2_f32(Some(&position_location), sprite.position.x, sprite.position.y);

            // Bind the texture
            gl.bind_texture(TEXTURE_2D, Some(NativeTexture(NonZeroU32::new(sprite.texture_id).unwrap())));

            // Define sprite vertex data
            let vertices = [
                -sprite.size.x / 2.0, -sprite.size.y / 2.0, 0.0, 0.0, sprite.colour[0], sprite.colour[1], sprite.colour[2], sprite.colour[3], // Bottom-left
                sprite.size.x / 2.0, -sprite.size.y / 2.0, 1.0, 0.0, sprite.colour[0], sprite.colour[1], sprite.colour[2], sprite.colour[3], // Bottom-right
                sprite.size.x / 2.0, sprite.size.y / 2.0, 1.0, 1.0, sprite.colour[0], sprite.colour[1], sprite.colour[2], sprite.colour[3], // Top-right
                -sprite.size.x / 2.0, sprite.size.y / 2.0, 0.0, 1.0, sprite.colour[0], sprite.colour[1], sprite.colour[2], sprite.colour[3], // Top-left
            ];

            // Define indices
            let indices = [0, 1, 2, 2, 3, 0];

            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), STATIC_DRAW);

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices), STATIC_DRAW);

            let stride = (2 + 2 + 4) * std::mem::size_of::<f32>() as i32; // Position + TexCoords + Color
            gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, stride, (2 * std::mem::size_of::<f32>()) as i32);
            gl.enable_vertex_attrib_array(1);

            gl.vertex_attrib_pointer_f32(2, 4, FLOAT, false, stride, (4 * std::mem::size_of::<f32>()) as i32);
            gl.enable_vertex_attrib_array(2);

            // Draw the rotated sprite
            gl.draw_elements(TRIANGLES, indices.len() as i32, UNSIGNED_INT, 0);

            gl.delete_vertex_array(vao);
            gl.delete_buffer(vbo);
            gl.delete_buffer(ebo);
        }
    }
}
