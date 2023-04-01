use micromath::F32Ext;

pub fn sqrt(args: super::Arguments) -> f32 {
    return args.arg(0).sqrt();
}

// how could i not
// https://github.com/id-Software/Quake-III-Arena/blob/master/code/game/q_math.c#L552-L564
#[allow(non_snake_case)]
pub fn Q_rsqrt(args: super::Arguments) -> f32 {
    let number = args.arg(0);


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