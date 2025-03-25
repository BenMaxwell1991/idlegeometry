use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::{Pos2FixedPoint, INVALID_POSITION};
use crate::game::objects::animation::Animation;
use crate::game::objects::attacks::attack_stats::{AttackName, AttackStats};
use crate::game::objects::loot::Loot;
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use crate::game::objects::on_death::OnDeath;
use crate::game::objects::unit_defaults::collectable_01_basic_monster;
use crate::game::objects::upgrades::{Upgrade, UpgradeType};
use crate::helper::lock_helper::acquire_lock_mut;
use rayon::iter::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::mem::swap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct GameObject {
    pub id: u32,
    pub object_type: ObjectType,
    pub object_shape: ObjectShape,
    pub move_speed: i32,
    pub health_max: f32,
    pub health_current: f32,
    pub animation: Option<Animation>,
    pub attack_cooldowns: FxHashMap<AttackName, f32>,
    pub upgrades: Vec<Upgrade>,
    pub pickup_radius: Option<i32>,
    pub loot: Option<Loot>,
    pub on_death: OnDeath,

    pub parent_unit_id: Option<u32>,
    pub attack_stats: Option<AttackStats>,
}

impl GameObject {
    pub fn new(object_type: ObjectType, object_shape: ObjectShape, move_speed: i32, health_max: f32, health_current: f32, animation: Option<Animation>) -> Self {
        Self {
            id: u32::MAX,
            object_type,
            object_shape,
            move_speed,
            health_max,
            health_current,
            animation,
            attack_cooldowns: FxHashMap::default(),
            upgrades: Vec::new(),
            pickup_radius: None,
            loot: None,
            on_death: OnDeath::default(),
            parent_unit_id: None,
            attack_stats: None,
        }
    }
    pub fn apply_damage(&mut self, damage: f64) -> bool {
        if let Some(mut animation) = self.animation.as_mut() {
            self.health_current -= damage as f32;
            animation.last_damage_time = Some(Instant::now());
            self.health_current <= 0.0
        } else {
            false
        }
    }
}

pub fn add_units(units: Vec<GameObject>, positions: Vec<Pos2FixedPoint>, game_data: &GameData) {
    let mut game_units = acquire_lock_mut(&game_data.units, "Failed to acquire game_units lock");
    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "Failed to acquire unit_positions lock");
    let mut empty_indexes = acquire_lock_mut(&game_data.empty_unit_indexes, "Failed to acquire empty_unit_indexes lock");
    let mut spatial_grid = acquire_lock_mut(&game_data.spatial_hash_grid, "Failed to acquire spatial_grid lock");

    for (mut unit, position) in units.into_iter().zip(positions.into_iter()) {
        let unit_id = if let Some(reuse_index) = empty_indexes.pop() {
            unit.id = reuse_index;
            game_units[reuse_index as usize] = Some(unit.clone());
            unit_positions[reuse_index as usize] = position;
            reuse_index
        } else {
            let new_index = game_units.len() as u32;
            unit.id = new_index;
            game_units.push(Some(unit.clone()));
            unit_positions.push(position);
            new_index
        };

        spatial_grid.insert_unit(unit_id, position);
    }
}

pub fn remove_units(unit_ids: Vec<u32>, game_data: Arc<GameData>) -> Vec<(GameObject, Pos2FixedPoint)> {
    let mut game_units = acquire_lock_mut(&game_data.units, "Failed to acquire game_units lock");
    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "Failed to acquire unit_positions lock");
    let mut spatial_grid = acquire_lock_mut(&game_data.spatial_hash_grid, "Failed to acquire spatial_grid lock");
    let mut empty_indexes = acquire_lock_mut(&game_data.empty_unit_indexes, "Failed to acquire empty_unit_indexes lock");

    let mut collectables_to_spawn = Vec::new();
    let mut sounds_to_play = FxHashSet::default();

    for &unit_id in &unit_ids {
        if let Some(unit) = game_units.get_mut(unit_id as usize) {
            if let Some(unit) = unit.take() {
                let position = unit_positions[unit_id as usize];

                if let Some(sound) = &unit.on_death.sound {
                    sounds_to_play.insert(sound.clone());
                }

                if unit.object_type == ObjectType::Player {
                    *acquire_lock_mut(&game_data.player_dead, "player_dead") = true;
                    *acquire_lock_mut(&game_data.game_state, "game_state") = GameState::Dead;
                }

                // If the unit is an enemy with loot, return a collectable
                if unit.object_type == ObjectType::Enemy {
                    if let Some(loot) = unit.loot {
                        let collectable = collectable_01_basic_monster(Some(loot));
                        collectables_to_spawn.push((collectable, position));
                    }
                }

                // Remove the unit itself
                spatial_grid.remove_unit(&unit_id, position);
                unit_positions[unit_id as usize] = Pos2FixedPoint::new(INVALID_POSITION, INVALID_POSITION);
                empty_indexes.push(unit_id);
            }
        }
    }

    for sound in sounds_to_play {
        // play_sound(Arc::clone(&game_data), &sound.name, sound.volume);
    }

    collectables_to_spawn
}

pub fn move_units_batched(unit_positions_updates: &[(u32, Pos2FixedPoint, Pos2FixedPoint)], game_data: &GameData, player_id: Option<u32>) {
    let mut game_units = acquire_lock_mut(&game_data.units, "game_units");
    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "unit_positions");
    let mut spatial_grid = acquire_lock_mut(&game_data.spatial_hash_grid, "spatial_hash_grid");
    let mut camera_state = acquire_lock_mut(&game_data.camera_state, "camera_state");

    unit_positions.clear();
    let mut new_positions: Vec<Pos2FixedPoint> = Vec::with_capacity(unit_positions_updates.len());
    new_positions = unit_positions_updates.par_iter().map(|&(_, _, new_pos)| new_pos).collect();
    swap(&mut *unit_positions, &mut new_positions);
    spatial_grid.update_units_position_in_grid(unit_positions_updates);

    if let Some(id) = player_id {
        if let Some((_, _, new_pos)) = unit_positions_updates.iter().find(|&&(unit_id, _, _)| unit_id == id) {
            let mut player_position_lock = game_data.player_position.write().unwrap();
            *player_position_lock = Some(*new_pos);
            camera_state.set_target(*new_pos);
            camera_state.move_to_target();
        }
    }
}

pub fn apply_upgrade(unit: &mut GameObject, upgrade_type: UpgradeType) {
    if let Some(existing_upgrade) = unit.upgrades.iter_mut().find(|u| u.upgrade_type == upgrade_type) {
        existing_upgrade.level += 1;
    } else {
        unit.upgrades.push(Upgrade { upgrade_type, level: 1 });
    }

    println!("Upgrade Applied: {:?}", unit.upgrades);
}

