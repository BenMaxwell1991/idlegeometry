use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::collision::detect_collision::handle_terrain;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, GAME_IN_FOCUS, KEY_STATE};
use crate::game::loops::game_loop::get_player_position;
use crate::game::loops::key_state::KeyState;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn move_player(game_data: Arc<GameData>) {
    let mut delta_time = 0.0;
    loop {
        if !game_data.game_loop_active.load(Ordering::Relaxed) {
        sleep(Duration::from_millis(10));
            continue;
        }

        let now = Instant::now();
        let current_tab = game_data.get_field(CURRENT_TAB).unwrap_or(NullGameTab).clone();
        let in_focus = game_data.get_field(GAME_IN_FOCUS).unwrap_or(false).clone();
        let key_state = game_data.get_field(KEY_STATE).unwrap_or(Arc::new(KeyState::new())).clone();
        let (player_id, player_position) = get_player_position(&game_data).clone();

        let mut game_units = acquire_lock_mut(&game_data.units, "game_units");
        if let Some(player_id) = player_id {
            if let Some(Some(player_unit)) = game_units.get_mut(player_id as usize) {
                let movement_speed = player_unit.move_speed;
                let shape = player_unit.object_shape.clone();
                drop(game_units);
                let distance: f32 = movement_speed as f32 * delta_time as f32;
                let mut new_position = player_position;

                if current_tab == GameTab::Adventure && in_focus {
                    let dx = key_state.d.load(Ordering::Relaxed) as i32 - key_state.a.load(Ordering::Relaxed) as i32;
                    let dy = key_state.s.load(Ordering::Relaxed) as i32 - key_state.w.load(Ordering::Relaxed) as i32;

                    new_position.x += dx * distance as i32;
                    new_position.y += dy * distance as i32;
                }

                let game_map = acquire_lock(&game_data.game_map, "game_map");
                let tile_size = game_map.as_ref().map(|m| m.get_tile_size()).unwrap_or(1);
                if let Some(game_map) = game_map.as_ref() {
                    handle_terrain(&mut new_position, &player_position, &shape, game_map, tile_size);
                }
                drop(game_map);

                if new_position != player_position {
                    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "unit_positions");
                    let mut spatial_hash = acquire_lock_mut(&game_data.spatial_hash_grid, "spatial_hash");
                    let mut camera_state = acquire_lock_mut(&game_data.camera_state, "camera_state");
                    spatial_hash.move_unit(player_id, player_position, new_position);
                    if let Some(mut pos) = unit_positions.get_mut(player_id as usize) {
                        *pos = new_position;
                    }
                    camera_state.set_target(new_position);
                    camera_state.move_to_target();
                    drop(unit_positions);
                    drop(spatial_hash);
                    drop(camera_state);
                }

            }
        }
        sleep(Duration::from_millis(8));
        delta_time = now.elapsed().as_secs_f64();
    }
}