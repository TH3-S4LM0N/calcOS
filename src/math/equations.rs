/*/ how could i not
// https://github.com/id-Software/Quake-III-Arena/blob/master/code/game/q_math.c#L552-L564
#[allow(non_snake_case)]
fn Q_sqrt(number: f64) -> f64 {
    let mut i: i32;
    let (x2, mut y): (f64, f64);
    const THREEHALVES: f64 = 1.5;

    x2 = number * 0.5;
    y = number;
    i = y.to_bits() as i32; // evil floating point bit level hacking
    i = 0x5fe6eb50c7b537a9 - (i >> 1); // what the heck?
    y = f64::from_bits(i as u64);
    y = y * (THREEHALVES - (x2 * y * y)); // 1st iteration
    // y = y * (threehalves - (x2 * y * y));   // 2nd iteration, this can be removed

    return y;
}*/