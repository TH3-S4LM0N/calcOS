use micromath::F32Ext;

pub mod triangles;

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

/// Solves a proportion: \
/// ```
/// x b
/// a c
/// ```
/// where x is the return value
pub fn proportion(a: f32, b: f32, c: f32) -> f32 {
    if c > a {
        //factor = c / a;
        return b / (c / a);
    } else {
        return b * (a / c);
    }
}

#[test_case]
fn test_prop() {
    let proprotion = proportion(12.0, 2.0, 4.0);

    assert_eq!(proprotion, 6.0);
}