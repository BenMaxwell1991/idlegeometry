use std::num::NonZeroU32;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{GAME_MAP, SPRITE_SHEETS_NATIVE};
use crate::game::map::camera_state::CameraState;
use crate::game::map::tile_type::TileType;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::units::unit_type::UnitType;
use crate::ui::component::widget::game_graphics::world_to_screen;
use eframe::emath::{Rect, Vec2};
use egui::{Color32, Pos2};
use glow::*;
use std::ops::Add;
use rustc_hash::FxHashMap;
use crate::ui::graphics::offscreen_renderer::OffscreenRenderer;

pub fn draw_map(gl: &Context, game_data: &GameData, paintbox_rect: &Rect, camera_state: &CameraState) {
    if let Some(game_map) = game_data.get_field(GAME_MAP) {
        let tile_size = camera_state.get_zoom_scaled() * game_map.tile_size as f32 / FIXED_POINT_SCALE as f32;

        let mut rects = Vec::new();
        let mut colours = Vec::new();

        for (&(x, y), tile) in &game_map.tiles {
            let world_pos = Pos2FixedPoint::new(x as i32 * game_map.tile_size, y as i32 * game_map.tile_size);
            let screen_pos = world_to_screen(world_pos, camera_state, paintbox_rect);
            let tile_rect = Rect::from_min_size(screen_pos, Vec2::new(tile_size, tile_size));

            let colour = match tile.tile_type {
                TileType::Wall => Color32::from_rgb(100, 100, 100),
                TileType::SpawnPoint => Color32::from_rgb(0, 0, 100),
                TileType::Empty => Color32::from_rgb(0, 0, 0),
            };

            rects.push(tile_rect);
            colours.push(colour);
        }

        if let shader_lock = game_data.rect_shader.write().unwrap() {
            draw_colour_rectangles(gl, &paintbox_rect, &rects, &colours, &*shader_lock);
        }
    }
}

pub fn draw_units(gl: &Context, game_data: &GameData, paintbox_rect: &Rect, camera_state: &CameraState) {
    let sprite_sheets = game_data.get_field(SPRITE_SHEETS_NATIVE);
    let units_lock = game_data.units.read().unwrap();
    let unit_positions_lock = game_data.unit_positions.read().unwrap();

    let mut images_to_draw = Vec::new();
    let mut rects_to_draw = Vec::new();
    let mut colours_to_draw = Vec::new();
    let mut player_to_draw = Vec::new();
    let mut health_bar_rects = Vec::new();
    let mut health_bar_colours = Vec::new();

    for unit_option in units_lock.iter() {
        if let Some(unit) = unit_option {
            let unit_screen_position = world_to_screen(unit_positions_lock[unit.id as usize], camera_state, paintbox_rect);

            // Skip rendering if unit is out of view
            if !paintbox_rect.contains(unit_screen_position.add(paintbox_rect.min.to_vec2())) {
                continue;
            }

            // Scale unit size based on zoom
            let unit_size = Vec2::new(40.0, 40.0) * camera_state.get_zoom_scaled();
            let unit_rect = Rect::from_center_size(unit_screen_position, unit_size);

            // If unit is small and not a player, draw as a rectangle
            if (unit_size.x < 5.0 || unit_size.y < 5.0) && unit.unit_type != UnitType::Player {
                rects_to_draw.push(unit_rect);
                colours_to_draw.push(Color32::RED); // You can change colors based on unit type
                continue;
            }


            // If the unit has a valid sprite, use it
            if let Some(sprite_sheets) = sprite_sheets.as_ref() {
                if let Some(sprite_sheet) = sprite_sheets.get(&unit.animation.sprite_key) {
                    let frame_index = (unit.animation.animation_frame * sprite_sheet.get_frame_count_native() as f32).trunc() as usize;
                    let frame = sprite_sheet.get_frame_native(frame_index);

                    match unit.unit_type {
                        UnitType::Player => {
                            player_to_draw.push((frame, unit_rect));

                            let health_bar_height = 4.0 * camera_state.get_zoom_scaled();
                            let health_bar_width = unit_size.x * 0.9;
                            let current_health_width = health_bar_width * (unit.health_current / unit.health_max);

                            let health_bar_bg_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);
                            let health_bar_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);

                            let health_bar_bg_rect = Rect::from_min_size(health_bar_bg_min, Vec2::new(health_bar_width, health_bar_height));
                            let health_bar_rect = Rect::from_min_size(health_bar_min, Vec2::new(current_health_width, health_bar_height));

                            health_bar_rects.push(health_bar_bg_rect);
                            health_bar_colours.push(Color32::BLACK); // Background
                            health_bar_rects.push(health_bar_rect);
                            health_bar_colours.push(Color32::GREEN); // Health
                        },
                        UnitType::Enemy => {
                            println!("Unit Id: {}", unit.id);
                            println!("Frame_Number: {}", frame_index);
                            images_to_draw.push((frame, unit_rect));

                            if unit.health_current != unit.health_max {
                                let health_bar_height = 3.0 * camera_state.get_zoom_scaled();
                                let health_bar_width = unit_size.x * 0.7;
                                let current_health_width = health_bar_width * (unit.health_current / unit.health_max);

                                let health_bar_bg_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);
                                let health_bar_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);

                                let health_bar_bg_rect = Rect::from_min_size(health_bar_bg_min, Vec2::new(health_bar_width, health_bar_height));
                                let health_bar_rect = Rect::from_min_size(health_bar_min, Vec2::new(current_health_width, health_bar_height));

                                health_bar_rects.push(health_bar_bg_rect);
                                health_bar_colours.push(Color32::BLACK); // Background
                                health_bar_rects.push(health_bar_rect);
                                health_bar_colours.push(Color32::RED); // Health
                            }
                        }
                    }
                }
            }
        }
    }

    if let shader_lock = game_data.rect_shader.write().unwrap() {
        draw_colour_rectangles(gl, &paintbox_rect, &rects_to_draw, &colours_to_draw, &*shader_lock);
    }

    if let shader_lock = game_data.sprite_shader.write().unwrap() {
        draw_colour_sprites(gl, paintbox_rect, &images_to_draw, &*shader_lock);
    }

    if let shader_lock = game_data.sprite_shader.write().unwrap() {
        draw_colour_sprites(gl, paintbox_rect, &player_to_draw, &*shader_lock);
    }

    if let shader_lock = game_data.rect_shader.write().unwrap() {
        draw_colour_rectangles(gl, &paintbox_rect, &health_bar_rects, &health_bar_colours, &*shader_lock);
    }
}

pub fn draw_colour_sprites(
    gl: &Context,
    view_rect: &Rect,
    sprites: &[(NativeTexture, Rect)], // (Texture ID, Rect)
    shader: &Option<NativeProgram>
) {
    unsafe {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        let mut sprite_batches: FxHashMap<u32, Vec<Rect>> = FxHashMap::default();
        for (texture_id, rect) in sprites.iter() {
            sprite_batches.entry(texture_id.0.get()).or_default().push(*rect);
        }

        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();

        gl.use_program(*shader);
        gl.bind_vertex_array(Some(vao));

        for (texture_id, batch) in sprite_batches {
            vertices.clear();
            indices.clear();
            index_offset = 0;

            for rect in batch.iter() {
                let (min_pos, size) = get_gl_rect(&view_rect, rect);
                let tex_coords = [0.0, 0.0, 1.0, 1.0];

                let sprite_vertices = [
                    min_pos.x,           min_pos.y,            tex_coords[0], tex_coords[1], // Bottom-left
                    min_pos.x + size.x,  min_pos.y,            tex_coords[2], tex_coords[1], // Bottom-right
                    min_pos.x + size.x,  min_pos.y + size.y,   tex_coords[2], tex_coords[3], // Top-right
                    min_pos.x,           min_pos.y + size.y,   tex_coords[0], tex_coords[3], // Top-left
                ];

                vertices.extend_from_slice(&sprite_vertices);
                indices.extend_from_slice(&[
                    index_offset, index_offset + 1, index_offset + 2,
                    index_offset + 2, index_offset + 3, index_offset
                ]);

                index_offset += 4;
            }

            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), STATIC_DRAW);

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices), STATIC_DRAW);

            let stride = (2 + 2) * size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, stride, (2 * size_of::<f32>()) as i32);
            gl.enable_vertex_attrib_array(1);

            let texture = NativeTexture(NonZeroU32::try_from(texture_id).unwrap());
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.draw_elements(TRIANGLES, indices.len() as i32, UNSIGNED_INT, 0);
        }

        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
        gl.delete_buffer(ebo);
    }
}


pub fn get_gl_rect(view: &Rect, rect: &Rect) -> (Pos2, Vec2) {
    let min_x= (rect.min.x / view.size().x) * 2.0 - 1.0;
    let min_y= (rect.min.y / view.size().y) * 2.0 - 1.0;

    let w = (rect.size().x / view.size().x) * 2.0;
    let h = (rect.size().y / view.size().y) * 2.0;

    (Pos2::new(min_x, min_y), Vec2::new(w, h))
}


pub fn get_vertex_from_gl_rect(min_pos: Pos2, size: Vec2, rgba: Color32) -> [f32; 24] {
    let r = rgba.r() as f32 / 255.0;
    let g = rgba.g() as f32 / 255.0;
    let b = rgba.b() as f32 / 255.0;
    let a = rgba.a() as f32 / 255.0;

    [
        min_pos.x,           min_pos.y,            r, g, b, a, // Bottom-left
        min_pos.x + size.x,  min_pos.y,            r, g, b, a, // Bottom-right
        min_pos.x + size.x,  min_pos.y + size.y,   r, g, b, a, // Top-right
        min_pos.x,           min_pos.y + size.y,   r, g, b, a, // Top-left
    ]
}

pub fn draw_colour_rectangles(gl: &Context, view_rect: &Rect, rectangles: &[Rect], colours: &[Color32], shader: &Option<NativeProgram>) {
    unsafe {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        for (i, rect) in rectangles.iter().enumerate() {
            let (min_pos, size) = get_gl_rect(&view_rect, &rect);
            let rect_vertices = get_vertex_from_gl_rect(min_pos, size, colours[i]);
            vertices.extend_from_slice(&rect_vertices);
            indices.extend_from_slice(&[
                index_offset, index_offset + 1, index_offset + 2,
                index_offset + 2, index_offset + 3, index_offset
            ]);
            index_offset += 4;
        }

        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();

        gl.use_program(*shader);

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), STATIC_DRAW);

        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices), STATIC_DRAW);

        let stride = (2 + 4) * size_of::<f32>() as i32; // 2 Position + 4 Color
        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 4, FLOAT, false, stride, (2 * std::mem::size_of::<f32>()) as i32);
        gl.enable_vertex_attrib_array(1);

        gl.draw_elements(TRIANGLES, indices.len() as i32, UNSIGNED_INT, 0);

        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
        gl.delete_buffer(ebo);
    }
}

pub fn create_sprite_shader_program(gl: &Context) -> NativeProgram {
    unsafe {
        let vertex_shader_source = r#"
            #version 330 core
            layout (location = 0) in vec2 aPos;
            layout (location = 1) in vec2 aTexCoord;

            out vec2 TexCoord;

            void main() {
                gl_Position = vec4(aPos, 0.0, 1.0);
                TexCoord = aTexCoord;
            }
        "#;

        let fragment_shader_source = r#"
            #version 330 core
            in vec2 TexCoord;
            out vec4 FragColor;

            uniform sampler2D spriteTexture;

            void main() {
                FragColor = texture(spriteTexture, TexCoord);
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

        gl.detach_shader(shader_program, vertex_shader);
        gl.detach_shader(shader_program, fragment_shader);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        shader_program
    }
}

pub fn create_rect_shader_program(gl: &Context) -> NativeProgram {
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

        gl.detach_shader(shader_program, vertex_shader);
        gl.detach_shader(shader_program, fragment_shader);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        shader_program
    }
}