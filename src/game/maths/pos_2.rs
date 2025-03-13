use crate::game::maths::integers::fast_inverse_sqrt_f32;

pub const INVALID_POSITION: i32 = i32::MIN; // -2,147,483,648
pub const FIXED_POINT_SCALE: i32 = 1024;
pub const FIXED_POINT_SHIFT: i32 = 10;


#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Pos2FixedPoint {
    pub x: i32,
    pub y: i32,
}

impl Pos2FixedPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_f32(x: f32, y: f32) -> Self {
        Self {
            x: (x * FIXED_POINT_SCALE as f32) as i32,
            y: (y * FIXED_POINT_SCALE as f32) as i32,
        }
    }

    pub fn to_f32(self) -> (f32, f32) {
        (
            self.x as f32 / FIXED_POINT_SCALE as f32,
            self.y as f32 / FIXED_POINT_SCALE as f32,
        )
    }

    #[inline(always)]
    pub fn add(&self, other: Pos2FixedPoint) -> Self {
        Self {
            x: self.x.wrapping_add(other.x),
            y: self.y.wrapping_add(other.y),
        }
    }

    #[inline(always)]
    pub fn sub(&self, other: Pos2FixedPoint) -> Self {
        Self {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y),
        }
    }
}


#[inline(always)]
pub fn mul_i64(pos: Pos2FixedPoint, scalar: i64) -> (i64, i64) {
    (
        pos.x as i64 * scalar,
        pos.y as i64 * scalar,
    )
}

#[inline(always)]
pub fn div_i64(pos: (i64, i64), scalar: i32) -> (i64, i64) {
    if scalar == 0 {
        return pos; // Prevent division by zero
    }
    (
        pos.0 / scalar as i64,
        pos.1 / scalar as i64,
    )
}

#[inline(always)]
pub fn dot_i64(a: Pos2FixedPoint, b: Pos2FixedPoint) -> i64 {
    (a.x as i64 * b.x as i64) + (a.y as i64 * b.y as i64)
}

#[inline(always)]
pub fn magnitude_squared_i64(pos: Pos2FixedPoint) -> i64 {
    (pos.x as i64 * pos.x as i64) + (pos.y as i64 * pos.y as i64)
}

pub fn normalize_i64_upscaled(pos: Pos2FixedPoint) -> (i64, i64) {
    let mag_sq = magnitude_squared_i64(pos);
    if mag_sq == 0 {
        return (0, 0);
    }

    let inv_sqrt = fast_inverse_sqrt_f32(mag_sq as f32) * FIXED_POINT_SCALE as f32;

    (
        (pos.x as f32 * inv_sqrt) as i64,
        (pos.y as f32 * inv_sqrt) as i64,
    )
}

pub fn project_onto_i64(a: Pos2FixedPoint, normal: Pos2FixedPoint) -> (i64, i64) {
    let dot = dot_i64(a, normal);
    let normal_mag_sq = magnitude_squared_i64(normal);
    if normal_mag_sq == 0 {
        return (0, 0);
    }

    let (scaled_x, scaled_y) = mul_i64(normal, dot);

    let projected_x = scaled_x / normal_mag_sq;
    let projected_y = scaled_y / normal_mag_sq;

    (projected_x, projected_y)
}