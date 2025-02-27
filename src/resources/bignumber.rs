use crate::enums::numberformatmode::NumberFormatMode;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BigNumber {
    pub mantissa: f64,
    pub exponent: i64,
}

impl BigNumber {
    pub fn new(value: f64) -> Self {
        if value == 0.0 {
            return Self { mantissa: 0.0, exponent: 0 };
        }

        let mut mantissa = value;
        let mut exponent = 0;

        // Normalize: Ensure mantissa is in range [1, 10)
        while mantissa.abs() < 1.0 {
            mantissa *= 10.0;
            exponent -= 1;
        }
        while mantissa.abs() >= 10.0 {
            mantissa /= 10.0;
            exponent += 1;
        }

        Self { mantissa, exponent }
    }

    pub fn format_number(&self, mode: NumberFormatMode) -> String {
        match mode {
            NumberFormatMode::Standard => {
                if self.exponent < 6 {
                    format!("{:.2}", self.mantissa * 10f64.powi(self.exponent as i32))
                } else {
                    format!("{:.5}e{}", self.mantissa, self.exponent)
                }
            }
            NumberFormatMode::Scientific => {
                format!("{:.3e}", self.mantissa * 10f64.powi(self.exponent as i32))
            }
            NumberFormatMode::Exponential => {
                format!("{:.5}e{}", self.mantissa, self.exponent)
            }
        }
    }

    /// Normalizes mantissa to be within [1, 10)
    fn normalize(&mut self) {
        while self.mantissa.abs() < 1.0 && self.mantissa != 0.0 {
            self.mantissa *= 10.0;
            self.exponent -= 1;
        }
        while self.mantissa.abs() >= 10.0 {
            self.mantissa /= 10.0;
            self.exponent += 1;
        }
    }
}

// Implement `+=` for BigNumber (Addition)
impl AddAssign for BigNumber {
    fn add_assign(&mut self, other: Self) {
        if self.mantissa == 0.0 {
            *self = other;
            return;
        }
        if other.mantissa == 0.0 {
            return;
        }

        let (mut big, mut small) = if self.exponent >= other.exponent {
            (*self, other)
        } else {
            (other, *self)
        };

        // Scale the smaller number's mantissa to match the larger exponent
        let exponent_diff = big.exponent - small.exponent;
        if exponent_diff > 15 {
            return;
        }

        small.mantissa /= 10f64.powi(exponent_diff as i32);
        big.mantissa += small.mantissa;

        *self = big;
        self.normalize();
    }
}

// Implement `+` for BigNumber (Addition)
impl Add for BigNumber {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

// Implement `-=` for BigNumber (Subtraction)
impl SubAssign for BigNumber {
    fn sub_assign(&mut self, other: Self) {
        if self.mantissa == 0.0 {
            *self = other;
            return;
        }
        if other.mantissa == 0.0 {
            return;
        }

        let (mut big, mut small) = if self.exponent >= other.exponent {
            (*self, other)
        } else {
            (other, *self)
        };

        // Scale the smaller number's mantissa to match the larger exponent
        let exponent_diff = big.exponent - small.exponent;
        if exponent_diff > 15 {
            return;
        }

        small.mantissa /= 10f64.powi(exponent_diff as i32);
        big.mantissa -= small.mantissa;

        *self = big;
        self.normalize();
    }
}

// Implement `-` for BigNumber (Subtraction)
impl Sub for BigNumber {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

// Implement `*=` for BigNumber (Multiplication)
impl MulAssign for BigNumber {
    fn mul_assign(&mut self, other: Self) {
        self.mantissa *= other.mantissa;
        self.exponent += other.exponent;
        self.normalize();
    }
}

// Implement `*` for BigNumber (Multiplication)
impl Mul for BigNumber {
    type Output = Self;
    fn mul(mut self, other: Self) -> Self {
        self *= other;
        self
    }
}

// Implement `/=` for BigNumber (Division)
impl DivAssign for BigNumber {
    fn div_assign(&mut self, other: Self) {
        if other.mantissa == 0.0 {
            panic!("Cannot divide by zero!");
        }

        self.mantissa /= other.mantissa;
        self.exponent -= other.exponent;
        self.normalize();
    }
}

// Implement `/` for BigNumber (Division)
impl Div for BigNumber {
    type Output = Self;
    fn div(mut self, other: Self) -> Self {
        self /= other;
        self
    }
}