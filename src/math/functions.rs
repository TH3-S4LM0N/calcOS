use crate::println;

// how could i not
// https://github.com/id-Software/Quake-III-Arena/blob/master/code/game/q_math.c#L552-L564
#[allow(non_snake_case)]
pub fn Q_sqrt(number: f32) -> f32 {
    let mut i: i32;
    let (x2, mut y): (f32, f32);
    const THREEHALVES: f32 = 1.5;

    x2 = number * 0.5;
    y = number;
    i = y.to_bits() as i32; // evil floating point bit level hacking
    i = 0x5f3759df - (i >> 1); // what the heck?
    y = f32::from_bits(i as u32);
    y = y * (THREEHALVES - (x2 * y * y)); // 1st iteration
    // y = y * (THREEHALVES - (x2 * y * y));   // 2nd iteration, this can be removed

    return y;
}

/// The Pythagorean Theorem: `a^2 + b^2 = c^2`.\
/// Where:
/// - a: first argument
/// - b: second argument
/// - c: return value, NOT c^2, just c \
/// This is fairly innacurate.
pub fn py_thrm(a: f32, b: f32) -> f32 {
    sqrt((a * a) + (b * b))
}

/// 5% accuracy
/// https://github.com/tarcieri/micromath/blob/main/src/float/sqrt.rs
pub fn sqrt(num: f32) -> f32 {
    if num.is_sign_negative() {todo!()}
    f32::from_bits((num.to_bits() + 0x3f80_0000) >> 1)
}