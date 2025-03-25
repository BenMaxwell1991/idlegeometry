use crate::game::map::tile_type::TileType;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::objects::object_type::ObjectType;
use crate::ui::asset::sprite::sprite_sheet::GRASS;
use crate::ui::component::widget::game_graphics::world_to_screen;
use crate::ui::graphics::offscreen_renderer::OffscreenRenderer;
use crate::ui::graphics::rendering_data::RenderData;
use crate::ui::graphics::sprite_to_draw::SpriteToDraw;
use eframe::emath::{Rect, Vec2};
use egui::{Color32, Pos2};
use glow::*;
use rustc_hash::FxHashMap;
use std::f32::consts::PI;
use std::num::NonZeroU32;
use std::time::Instant;

pub fn draw_map(gl: &Context, render_data: &RenderData, paintbox_rect: &Rect, renderer: &OffscreenRenderer) {
    let camera_state = &render_data.camera_state;
    if let Some(game_map) = &render_data.game_map {
        let tile_size = camera_state.get_zoom_scaled() * game_map.get_tile_size() as f32 / FIXED_POINT_SCALE as f32;

        let mut rects = Vec::new();
        let mut colours = Vec::new();
        let mut sprite_tiles = Vec::new();

        for x in 0..game_map.width {
            for y in 0..game_map.height {
                let tile = game_map.get_tile(x, y);
                let world_pos = Pos2FixedPoint::new(x as i32 * game_map.get_tile_size(), y as i32 * game_map.get_tile_size());
                let screen_pos = world_to_screen(world_pos, camera_state, paintbox_rect);
                let tile_rect = Rect::from_min_size(screen_pos, Vec2::new(tile_size, tile_size));

                if !tile_rect.intersects(Rect::from_min_size(Pos2::new(0.0, 0.0), paintbox_rect.size())) {
                    continue;
                }

                match tile.tile_type {
                    TileType::Wall => {
                        rects.push(tile_rect);
                        colours.push(Color32::from_rgb(100, 100, 100));
                    }
                    TileType::SpawnPoint => {
                        rects.push(tile_rect);
                        colours.push(Color32::from_rgb(0, 0, 90));
                    }
                    TileType::Empty => {
                        rects.push(tile_rect);
                        colours.push(Color32::from_rgb(0, 0, 0));
                    }
                    TileType::Grass => {
                        fn pseudo_random(x: i32, y: i32, seed: u32) -> u32 {
                            // Simple deterministic noise-like hash
                            let mut n = x.wrapping_mul(374761393).wrapping_add(y.wrapping_mul(668265263));
                            n = (n ^ (n >> 13)).wrapping_mul(1274126177);
                            (n ^ (n >> 16)).wrapping_add(seed as i32) as u32
                        }

                        let patch_size = 1;
                        let patch_x = x / patch_size;
                        let patch_y = y / patch_size;

                        let seed = 1337;

                        let grass_index = (pseudo_random(patch_x as i32, patch_y as i32, seed) % 17) as usize;

                        if let Some(sprite_sheet) = renderer.sprite_sheets.get(GRASS) {
                            let frame = sprite_sheet.get_frame_native(grass_index);
                            sprite_tiles.push(SpriteToDraw {
                                texture: frame,
                                rect: tile_rect,
                                tint: Color32::WHITE,
                                blend_target: Color32::WHITE,
                                colour_blend_amount: 0.0,
                                alpha_blend_amount: 0.0,
                                rotation: 0.0,
                            });
                        } else {
                            // fallback: draw as coloured rect if sprite missing
                            rects.push(tile_rect);
                            colours.push(Color32::from_rgb(10, 80, 10));
                        }
                    }
                }
            }
        }

        draw_colour_rectangles(gl, &paintbox_rect, &rects, &colours, &Some(renderer.rect_shader));
        draw_colour_sprites(gl, &paintbox_rect, &sprite_tiles, &Some(renderer.sprite_shader));
    }
}

fn get_colour_blend_amount(last_damage_time: Option<Instant>) -> f32 {
    let damage_visual_duration_millis = 100f32;
    let inverse = 1f32 / damage_visual_duration_millis;
    let offset = damage_visual_duration_millis / 4f32;

    let time =  if let Some(instant) = last_damage_time {
        instant.elapsed().as_millis() as f32 + offset
    } else {
        damage_visual_duration_millis + 1.0
    };

    let colour_blend_amount = if time < damage_visual_duration_millis {
        (PI * time * inverse).sin()
    } else {
        0.0
    };

    colour_blend_amount
}

pub fn draw_units(gl: &Context, render_data: &RenderData, paintbox_rect: &Rect, renderer: &OffscreenRenderer) {
    let sprite_sheets = &renderer.sprite_sheets;
    let game_units = &render_data.game_units;
    let unit_positions = &render_data.unit_positions;
    let camera_state = &render_data.camera_state;

    let mut images_to_draw = Vec::new();
    let mut rects_to_draw = Vec::new();
    let mut colours_to_draw = Vec::new();
    let mut player_to_draw = Vec::new();
    let mut health_bar_rects = Vec::new();
    let mut health_bar_colours = Vec::new();
    let mut shadow_sprites_to_draw = Vec::new();

    println!("game units len: {}", game_units.len());
    println!("unit_positions len: {}", unit_positions.len());

    for unit_option in game_units.iter() {
        if let Some(unit) = unit_option {
            if let Some(animation) = &unit.animation {
                let unit_screen_position = world_to_screen(unit_positions[unit.id as usize], &camera_state, paintbox_rect);
                let unit_size = Vec2::new(animation.size.0 as f32, animation.size.1 as f32) * camera_state.get_zoom_scaled();
                let unit_rect = Rect::from_center_size(unit_screen_position, unit_size);

                if !unit_rect.intersects(Rect::from_min_size(Pos2::new(0.0, 0.0), paintbox_rect.size())) {
                    continue;
                }

                if (unit_size.x < 5.0 || unit_size.y < 5.0) && unit.object_type != ObjectType::Player {
                    rects_to_draw.push(unit_rect);
                    colours_to_draw.push(Color32::RED);
                    continue;
                }

                if let Some(sprite_sheet) = sprite_sheets.get(&animation.sprite_key) {
                    let frame_index = animation.fixed_frame_index.unwrap_or_else(|| {
                        (animation.animation_frame * sprite_sheet.get_frame_count_native() as f32).trunc() as usize
                    });

                    let last_damage_taken = animation.last_damage_time.clone();
                    let frame = sprite_sheet.get_frame_native(frame_index);

                    let shadow_scale = 1.2;
                    let shadow_size = unit_size * Vec2::new(shadow_scale, shadow_scale * 0.4);
                    let shadow_offset = Vec2::new(unit_size.x * 0.07, unit_size.y * 0.35);
                    let shadow_rect = Rect::from_center_size(unit_screen_position + shadow_offset, shadow_size);

                    let mut offset = 0.0;
                    if let Some(animation_offset) = animation.rotation_offset {
                        offset = animation_offset;
                    }

                    match unit.object_type {
                        ObjectType::Player => {
                            player_to_draw.push(SpriteToDraw {
                                texture: frame,
                                rect: unit_rect,
                                tint: Color32::WHITE,
                                blend_target: Color32::WHITE,
                                colour_blend_amount: get_colour_blend_amount(last_damage_taken),
                                alpha_blend_amount: 1.0,
                                rotation: offset,
                            });

                            let health_bar_height = 4.0 * camera_state.get_zoom_scaled();
                            let health_bar_width = unit_size.x * 0.9;
                            let current_health_width = health_bar_width * (unit.health_current / unit.health_max);

                            let health_bar_bg_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);
                            let health_bar_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);

                            let health_bar_bg_rect = Rect::from_min_size(health_bar_bg_min, Vec2::new(health_bar_width, health_bar_height));
                            let health_bar_rect = Rect::from_min_size(health_bar_min, Vec2::new(current_health_width, health_bar_height));

                            health_bar_rects.push(health_bar_bg_rect);
                            health_bar_colours.push(Color32::BLACK);
                            health_bar_rects.push(health_bar_rect);
                            health_bar_colours.push(Color32::GREEN);
                        },
                        ObjectType::Enemy => {
                            images_to_draw.push(SpriteToDraw {
                                texture: frame,
                                rect: unit_rect,
                                tint: Color32::WHITE,
                                blend_target: Color32::WHITE,
                                colour_blend_amount: get_colour_blend_amount(last_damage_taken),
                                alpha_blend_amount: 0.0,
                                rotation: offset,
                            });

                            if unit.health_current != unit.health_max {
                                let health_bar_height = 3.0 * camera_state.get_zoom_scaled();
                                let health_bar_width = unit_size.x * 0.7;
                                let current_health_width = health_bar_width * (unit.health_current / unit.health_max).max(0.0);

                                let health_bar_bg_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);
                                let health_bar_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);

                                let health_bar_bg_rect = Rect::from_min_size(health_bar_bg_min, Vec2::new(health_bar_width, health_bar_height));
                                let health_bar_rect = Rect::from_min_size(health_bar_min, Vec2::new(current_health_width, health_bar_height));

                                health_bar_rects.push(health_bar_bg_rect);
                                health_bar_colours.push(Color32::BLACK);
                                health_bar_rects.push(health_bar_rect);
                                health_bar_colours.push(Color32::RED);
                            }
                        }
                        ObjectType::Collectable => {
                            images_to_draw.push(SpriteToDraw {
                                texture: frame,
                                rect: unit_rect,
                                tint: Color32::WHITE,
                                blend_target: Color32::WHITE,
                                colour_blend_amount: 0.0,
                                alpha_blend_amount: 0.0,
                                rotation: offset,
                            });

                            if unit.health_current != unit.health_max {
                                let health_bar_height = 3.0 * camera_state.get_zoom_scaled();
                                let health_bar_width = unit_size.x * 0.7;
                                let current_health_width = health_bar_width * (unit.health_current / unit.health_max).max(0.0);

                                let health_bar_bg_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);
                                let health_bar_min = unit_screen_position + Vec2::new(-health_bar_width / 2.0, -unit_size.y * 0.5);

                                let health_bar_bg_rect = Rect::from_min_size(health_bar_bg_min, Vec2::new(health_bar_width, health_bar_height));
                                let health_bar_rect = Rect::from_min_size(health_bar_min, Vec2::new(current_health_width, health_bar_height));

                                health_bar_rects.push(health_bar_bg_rect);
                                health_bar_colours.push(Color32::BLACK);
                                health_bar_rects.push(health_bar_rect);
                                health_bar_colours.push(Color32::RED);
                            }
                        }
                        ObjectType::Attack => {
                            if let Some(stats) = &unit.attack_stats {
                                offset += stats.direction.1.atan2(stats.direction.0).to_degrees();
                            }
                            images_to_draw.push(SpriteToDraw {
                                texture: frame,
                                rect: unit_rect,
                                tint: Color32::WHITE,
                                blend_target: Color32::WHITE,
                                colour_blend_amount: 0.0,
                                alpha_blend_amount: 0.0,
                                rotation: offset,
                            });
                        }
                    }
                    shadow_sprites_to_draw.push(SpriteToDraw {
                        texture: frame,
                        rect: shadow_rect,
                        tint: Color32::from_rgba_premultiplied(0, 0, 0, 192),
                        blend_target: Color32::WHITE,
                        colour_blend_amount: 0.0,
                        alpha_blend_amount: 0.0,
                        rotation: offset,
                    });
                }
            }
        }
    }

    draw_colour_sprites(gl, paintbox_rect, &shadow_sprites_to_draw, &Some(renderer.sprite_shader));
    draw_colour_rectangles(gl, paintbox_rect, &rects_to_draw, &colours_to_draw, &Some(renderer.rect_shader));
    draw_colour_sprites(gl, paintbox_rect, &images_to_draw, &Some(renderer.sprite_shader));
    draw_colour_rectangles(gl, paintbox_rect, &health_bar_rects, &health_bar_colours, &Some(renderer.rect_shader));
    draw_colour_sprites(gl, paintbox_rect, &player_to_draw, &Some(renderer.sprite_shader));
}

pub fn draw_colour_sprites(
    gl: &Context,
    view_rect: &Rect,
    sprites: &[SpriteToDraw],
    shader: &Option<NativeProgram>
) {
    unsafe {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        let mut sprite_batches: FxHashMap<u32, Vec<&SpriteToDraw>> = FxHashMap::default();
        for sprite in sprites.iter() {
            sprite_batches.entry(sprite.texture.0.get()).or_default().push(sprite);
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

            for sprite in batch.iter() {
                let tint = sprite.tint;
                let blend_target = sprite.blend_target;
                let colour_blend = sprite.colour_blend_amount;
                let alpha_blend = sprite.alpha_blend_amount;

                let screen_center = sprite.rect.center();
                let size = sprite.rect.size();

                let rotation_radians = sprite.rotation.to_radians();
                let cos = rotation_radians.cos();
                let sin = rotation_radians.sin();

                // Corners in local space relative to center
                let corners = [
                    (-size.x / 2.0, -size.y / 2.0, 0.0, 0.0), // Bottom-left
                    ( size.x / 2.0, -size.y / 2.0, 1.0, 0.0), // Bottom-right
                    ( size.x / 2.0,  size.y / 2.0, 1.0, 1.0), // Top-right
                    (-size.x / 2.0,  size.y / 2.0, 0.0, 1.0), // Top-left
                ];

                for (dx, dy, u, v) in corners {
                    let rotated_x = dx * cos - dy * sin;
                    let rotated_y = dx * sin + dy * cos;

                    let screen_x = screen_center.x + rotated_x;
                    let screen_y = screen_center.y + rotated_y;

                    let ndc_pos = to_gl_position(view_rect, Pos2::new(screen_x, screen_y));

                    vertices.extend_from_slice(&[
                        ndc_pos.x, ndc_pos.y, u, v,
                        tint.r() as f32 / 255.0,
                        tint.g() as f32 / 255.0,
                        tint.b() as f32 / 255.0,
                        tint.a() as f32 / 255.0,
                        colour_blend,
                        alpha_blend,
                        blend_target.r() as f32 / 255.0,
                        blend_target.g() as f32 / 255.0,
                        blend_target.b() as f32 / 255.0,
                        blend_target.a() as f32 / 255.0,
                    ]);
                }

                indices.extend_from_slice(&[
                    index_offset, index_offset + 1, index_offset + 2,
                    index_offset + 2, index_offset + 3, index_offset,
                ]);

                index_offset += 4;
            }

            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), STATIC_DRAW);

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices), STATIC_DRAW);

            let stride = (14) * std::mem::size_of::<f32>() as i32;

            gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, stride, 2 * 4);
            gl.enable_vertex_attrib_array(1);

            gl.vertex_attrib_pointer_f32(2, 4, FLOAT, false, stride, 4 * 4);
            gl.enable_vertex_attrib_array(2);

            gl.vertex_attrib_pointer_f32(3, 1, FLOAT, false, stride, 8 * 4); // colour_blend_amount
            gl.enable_vertex_attrib_array(3);

            gl.vertex_attrib_pointer_f32(4, 1, FLOAT, false, stride, 9 * 4); // alpha_blend_amount
            gl.enable_vertex_attrib_array(4);

            gl.vertex_attrib_pointer_f32(5, 4, FLOAT, false, stride, 10 * 4); // blend_target
            gl.enable_vertex_attrib_array(5);

            let texture = NativeTexture(NonZeroU32::try_from(texture_id).unwrap());
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.draw_elements(TRIANGLES, indices.len() as i32, UNSIGNED_INT, 0);
        }

        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
        gl.delete_buffer(ebo);
    }
}


pub fn draw_colour_spritess(
    gl: &Context,
    view_rect: &Rect,
    sprites: &[SpriteToDraw],
    shader: &Option<NativeProgram>
) {
    unsafe {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        let mut sprite_batches: FxHashMap<u32, Vec<&SpriteToDraw>> = FxHashMap::default();
        for sprite in sprites.iter() {
            sprite_batches.entry(sprite.texture.0.get()).or_default().push(sprite);
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

            for sprite in batch.iter() {
                let rect = &sprite.rect;
                let tint = sprite.tint;

                let (min_pos, size) = get_gl_rect(&view_rect, rect);
                let tex_coords = [0.0, 0.0, 1.0, 1.0];

                // Tint Colour
                let r = tint.r() as f32 / 255.0;
                let g = tint.g() as f32 / 255.0;
                let b = tint.b() as f32 / 255.0;
                let a = tint.a() as f32 / 255.0;

                // Blend target colour
                let tr = sprite.blend_target.r() as f32 / 255.0;
                let tg = sprite.blend_target.g() as f32 / 255.0;
                let tb = sprite.blend_target.b() as f32 / 255.0;
                let ta = sprite.blend_target.a() as f32 / 255.0;

                let colour_blend = sprite.colour_blend_amount;
                let alpha_blend = sprite.alpha_blend_amount;

                let sprite_vertices = [
                    min_pos.x,           min_pos.y,            tex_coords[0], tex_coords[1], r, g, b, a, colour_blend, alpha_blend, tr, tg, tb, ta, // Bottom-left
                    min_pos.x + size.x,  min_pos.y,            tex_coords[2], tex_coords[1], r, g, b, a, colour_blend, alpha_blend, tr, tg, tb, ta, // Bottom-right
                    min_pos.x + size.x,  min_pos.y + size.y,   tex_coords[2], tex_coords[3], r, g, b, a, colour_blend, alpha_blend, tr, tg, tb, ta, // Top-right
                    min_pos.x,           min_pos.y + size.y,   tex_coords[0], tex_coords[3], r, g, b, a, colour_blend, alpha_blend, tr, tg, tb, ta, // Top-left
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

            let stride = (14) * size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, stride, (2 * size_of::<f32>()) as i32);
            gl.enable_vertex_attrib_array(1);

            gl.vertex_attrib_pointer_f32(2, 4, FLOAT, false, stride, (4 * size_of::<f32>()) as i32);
            gl.enable_vertex_attrib_array(2);

            gl.vertex_attrib_pointer_f32(3, 1, FLOAT, false, stride, (8 * size_of::<f32>()) as i32); // colour_blend_amount
            gl.enable_vertex_attrib_array(3);

            gl.vertex_attrib_pointer_f32(4, 1, FLOAT, false, stride, (9 * size_of::<f32>()) as i32); // alpha_blend_amount
            gl.enable_vertex_attrib_array(4);

            gl.vertex_attrib_pointer_f32(5, 4, FLOAT, false, stride, 10 * size_of::<f32>() as i32); // Blend target colour
            gl.enable_vertex_attrib_array(5);

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

pub fn to_gl_position(view_rect: &Rect, point: Pos2) -> Pos2 {
    let x = (point.x / view_rect.size().x) * 2.0 - 1.0;
    let y = (point.y / view_rect.size().y) * 2.0 - 1.0;
    Pos2::new(x, y)
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
            layout (location = 2) in vec4 aColor;
            layout (location = 3) in float inColourBlendAmount;
            layout (location = 4) in float inAlphaBlendAmount;
            layout (location = 5) in vec4 inBlendTarget;

            out vec2 TexCoord;
            out vec4 VertexColor;
            out float vColourBlendAmount;
            out float vAlphaBlendAmount;
            out vec4 vBlendTarget;

            void main() {
                gl_Position = vec4(aPos, 0.0, 1.0);
                TexCoord = aTexCoord;
                VertexColor = aColor;
                vColourBlendAmount = inColourBlendAmount;
                vAlphaBlendAmount = inAlphaBlendAmount;
                vBlendTarget = inBlendTarget;
            }
        "#;

        let fragment_shader_source = r#"
            #version 330 core
            in vec2 TexCoord;
            in vec4 VertexColor;

            out vec4 FragColor;

            uniform sampler2D spriteTexture;
            in vec4 vBlendTarget;
            in float vColourBlendAmount;
            in float vAlphaBlendAmount;

            vec3 blendRGB(vec3 original, vec3 target, float amount) {
                return original * (1.0 - amount) + target * amount;
            }

            float blendAlpha(float original, float target, float amount) {
                return original * (1.0 - amount) + target * amount;
            }

            void main() {
                vec4 texColor = texture(spriteTexture, TexCoord);
                vec4 tintedColor = texColor * VertexColor;

                if (tintedColor.a < 0.01) {
                    discard;
                }

                vec3 blendedRGB = blendRGB(tintedColor.rgb, vBlendTarget.rgb, vColourBlendAmount);
                float blendedAlpha = blendAlpha(tintedColor.a, vBlendTarget.a, vAlphaBlendAmount);

                FragColor = vec4(blendedRGB, blendedAlpha);
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