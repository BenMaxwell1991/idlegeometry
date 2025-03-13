use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::GAME_MAP;
use crate::game::map::camera_state::CameraState;
use crate::game::map::tile_type::TileType;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::ui::component::widget::game_graphics::world_to_screen;
use eframe::emath::{Rect, Vec2};
use glow::*;
use std::sync::Arc;
use egui::{Color32, Pos2};

pub fn draw_map(gl: Arc<Context>, game_data: &GameData, rect: &Rect, camera_state: &CameraState) {
    let rects = [(Rect::from_center_size(Pos2::new(0.0, 0.0), Vec2::new(100.0, 200.0)))];
    let colours = [(Color32::RED)];

    draw_red_rectangle(gl.as_ref(), rect, &rects, &colours);   // if let Some(game_map) = game_data.get_field(GAME_MAP) {
    //     let tile_size = camera_state.get_zoom_scaled() * game_map.tile_size as f32 / FIXED_POINT_SCALE as f32;
    //     println!("Tile Size: {:?}", tile_size);
    //
    //     let mut vertices = Vec::new();
    //     let mut colors = Vec::new();
    //
    //     for (&(x, y), tile) in &game_map.tiles {
    //         let world_pos = Pos2FixedPoint::new(x as i32 * game_map.tile_size, y as i32 * game_map.tile_size);
    //         let screen_pos = world_to_screen(world_pos, camera_state, rect);
    //
    //         // println!("Screen pos: {:?}", screen_pos);
    //
    //         let tile_rect = Rect::from_min_size(screen_pos, Vec2::new(tile_size, tile_size));
    //
    //         // println!("tile_rect: {:?}", tile_rect);
    //
    //         let color = match tile.tile_type {
    //             TileType::Wall => [0.3, 0.3, 0.3, 1.0],
    //             TileType::SpawnPoint => [0.0, 0.0, 0.5, 1.0],
    //             TileType::Empty => [0.0, 0.0, 0.0, 0.0],
    //         };
    //
    //         // Normalize to OpenGL clip space (-1.0 to 1.0)
    //         let (x_min, y_min) = (
    //             (tile_rect.min.x / rect.width()) * 2.0 - 1.0,
    //             (tile_rect.min.y / rect.height()) * 2.0 - 1.0,
    //         );
    //         let (x_max, y_max) = (
    //             (tile_rect.max.x / rect.width()) * 2.0 - 1.0,
    //             (tile_rect.max.y / rect.height()) * 2.0 - 1.0,
    //         );
    //
    //         // println!(
    //         //     "🟡 Tile at ({}, {}): Screen Pos: {:?}, Normalized Rect: [{}, {}] -> [{}, {}]",
    //         //     x, y, screen_pos, x_min, y_min, x_max, y_max
    //         // );
    //
    //         // Two triangles to form a rectangle
    //         vertices.extend_from_slice(&[
    //             x_min, y_min, x_max, y_min, x_max, y_max,
    //             x_max, y_max, x_min, y_max, x_min, y_min,
    //         ]);
    //         colors.extend_from_slice(&[color; 6]); // 6 vertices per quad
    //     }
    //
    //     if !vertices.is_empty() {
    //         println!("✅ [draw_map] Generated {} vertices", vertices.len());
    //         unsafe {
    //             // Create VAO
    //             let vao = gl.create_vertex_array().unwrap();
    //             gl.bind_vertex_array(Some(vao));
    //
    //             // Create VBO (Vertex Buffer Object)
    //             let vbo = gl.create_buffer().unwrap();
    //             gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
    //             gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), STATIC_DRAW);
    //
    //             // Create CBO (Color Buffer Object)
    //             let cbo = gl.create_buffer().unwrap();
    //             gl.bind_buffer(ARRAY_BUFFER, Some(cbo));
    //             gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&colors), STATIC_DRAW);
    //
    //             // Use Shader Program
    //             let shader_program = create_shader_program(gl.as_ref());
    //             gl.use_program(Some(shader_program));
    //
    //             println!("✅ [draw_map] Shader Program ID: {}", shader_program.0.get());
    //
    //             // Enable Vertex Attribute for Position
    //             gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 0, 0);
    //             gl.enable_vertex_attrib_array(0);
    //
    //             // Enable Vertex Attribute for Color
    //             gl.vertex_attrib_pointer_f32(1, 4, FLOAT, false, 0, 0);
    //             gl.enable_vertex_attrib_array(1);
    //             println!("✅ [draw_map] Buffers and attributes set.");
    //
    //             println!("🟢 [draw_map] Drawing {} triangles", vertices.len() / 3);
    //             gl.draw_arrays(TRIANGLES, 0, (vertices.len() / 2) as i32);
    //             println!("✅ [draw_map] Called gl.draw_arrays()");
    //
    //             // Cleanup
    //             gl.delete_buffer(vbo);
    //             gl.delete_buffer(cbo);
    //             gl.delete_vertex_array(vao);
    //         }
    //     }
    // }
}

// Returns the GL position/size of a rect, given a viewing rect, and the rect to be drawns size
pub fn get_gl_rect(view_min: &Pos2, view_size: &Vec2, rect_size: &Vec2) -> (Pos2, Vec2) {
    let x = ((view_size.x + view_min.x - rect_size.x) / view_size.x) - 1.0;
    let y = 1.0 - (view_size.y - view_min.y - rect_size.y) / (view_size.y);
    let w = rect_size.x / (view_size.x / 2.0);
    let h = rect_size.y / (view_size.y / 2.0);
    (Pos2::new(x, y), Vec2::new(w, h))
}

// FINISH WRITING THIS FUNCTION!!!!!!
pub fn get_gl_recta(view: &Rect, rect: &Rect) -> (Pos2, Vec2) {
    let x = ((view.size().x + view.min.x - rect.size().x + rect.center().x) / view.size().x) - 1.0;
    let y = 1.0 - (view.size().y - view.min.y - rect.size().y + rect.center().y) / (view.size().y);
    let w = rect.size().x / (view.size().x / 2.0);
    let h = rect.size().y / (view.size().y / 2.0);
    (Pos2::new(x, y), Vec2::new(w, h))
}

pub fn get_vertex_from_gl_rect(center: Pos2, size: Vec2, rgba: Color32) -> [f32; 24] {
    let r = rgba.r() as f32 / 255.0;
    let g = rgba.g() as f32 / 255.0;
    let b = rgba.b() as f32 / 255.0;
    let a = rgba.a() as f32 / 255.0;
    [
        center.x,           center.y - size.y,  r, g, b, a, // Bottom-left (Red)
        center.x + size.x,  center.y - size.y,  r, g, b, a, // Bottom-right
        center.x + size.x,  center.y,           r, g, b, a, // Top-right
        center.x,           center.y,           r, g, b, a, // Top-left
    ]
}
pub fn draw_red_rectangle(gl: &Context, view_rect: &Rect, rects: &[Rect], colours: &[Color32]) {
    unsafe {
        let rect_width = 200.0;
        let rect_height = 100.0;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        for (i, rect) in rects.iter().enumerate() {
            // let (center, size) = get_gl_rect(&view_rect.min, &view_rect.size(), &Vec2::new(rect.width(), rect.height()));
            let (center, size) = get_gl_recta(&view_rect, &rect);
            let rect_vertices = get_vertex_from_gl_rect(center, size, colours[i]);
            vertices.extend_from_slice(&rect_vertices);

            // Indices for each rectangle
            indices.extend_from_slice(&[
                index_offset, index_offset + 1, index_offset + 2,
                index_offset + 2, index_offset + 3, index_offset
            ]);
            index_offset += 4; // 4 vertices per rectangle
        }


        // Convert UI pixels to OpenGL [-1,1] normalized device coordinates (NDC)
        // let (center, size) = get_gl_rect(&rect.min, &rect.size(), &Vec2::new(rect_width, rect_height));
        // let x = center.x;
        // let y = center.y;
        // let w = size.x;
        // let h = size.y;
        //
        // let vertices = get_vertex_from_gl_rect(center, size, Color32::RED);
        // let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();

        let shader_program = create_shader_program(gl);
        gl.use_program(Some(create_shader_program(gl)));

        gl.bind_vertex_array(Some(vao));

        // Upload vertex data
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), STATIC_DRAW);

        // Upload index data
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices), STATIC_DRAW);

        let stride = (2 + 4) * std::mem::size_of::<f32>() as i32; // 2 Position + 4 Color
        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 4, FLOAT, false, stride, (2 * std::mem::size_of::<f32>()) as i32);
        gl.enable_vertex_attrib_array(1);

        // ✅ Draw ALL rectangles in one call
        gl.draw_elements(TRIANGLES, indices.len() as i32, UNSIGNED_INT, 0);
        println!("✅ Drew {} rectangles in a single draw call.", rects.len());

        // Cleanup
        gl.delete_program(shader_program);
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
        gl.delete_buffer(ebo);
    }
}

pub fn create_shader_program(gl: &Context) -> NativeProgram {
    unsafe {
        let vertex_shader_source = r#"
            #version 330 core
            layout (location = 0) in vec2 aPos;
            layout (location = 1) in vec4 aColor;
            out vec4 fragColor;
            void main() {
                gl_Position = vec4(aPos, 0.0, 1.0);
                fragColor = aColor;
            }
        "#;

        let fragment_shader_source = r#"
            #version 330 core
            in vec4 fragColor;
            out vec4 FragColor;
            void main() {
                FragColor = fragColor;
            }
        "#;

        let vertex_shader = gl.create_shader(VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, vertex_shader_source);
        gl.compile_shader(vertex_shader);
        assert!(gl.get_shader_compile_status(vertex_shader));

        let fragment_shader = gl.create_shader(FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, fragment_shader_source);
        gl.compile_shader(fragment_shader);
        assert!(gl.get_shader_compile_status(fragment_shader));

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);
        gl.link_program(shader_program);
        assert!(gl.get_program_link_status(shader_program));

        shader_program
    }
}
