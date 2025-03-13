#[inline(always)]
pub fn int_sqrt_64(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }

    let mut x = n;
    let mut y = (x + 1) / 2;

    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }

    x
}

#[inline(always)]
pub fn fast_inverse_sqrt_f32(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }

    let half_x = 0.5 * x;
    let mut i: i32 = x.to_bits() as i32;
    i = 0x5f3759df - (i >> 1);
    let mut y = f32::from_bits(i as u32);

    y = y * (1.5 - (half_x * y * y));

    y
}
