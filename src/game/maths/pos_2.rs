pub const INVALID_POSITION: i32 = i32::MIN; // -2,147,483,648
pub const FIXED_POINT_SCALE: i32 = 1024;
pub const FIXED_POINT_SHIFT: i32 = 10;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Pos2FixedPoint {
    pub(crate) x: i32,
    pub(crate) y: i32,
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


    pub fn add(&self, other: Pos2FixedPoint) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: Pos2FixedPoint) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn bitshift_up(&self, bits: i32) -> Self {
        Self {
            x: self.x << bits,
            y: self.y << bits,
        }
    }

    pub fn bitshift_down(&self, bits: i32) -> Self {
        Self {
            x: self.x >> bits,
            y: self.y >> bits,
        }
    }
}
