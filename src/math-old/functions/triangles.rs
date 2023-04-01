use micromath::F32Ext;

/// The Pythagorean Theorem: `a^2 + b^2 = c^2`.\
/// Where:
/// - a: first argument
/// - b: second argument
/// - c: return value, NOT c^2, just c \
/// This is fairly innacurate as theres no good way to square root.
pub fn py_thrm(a: f32, b: f32) -> f32 {
    ((a * a) + (b * b)).sqrt()
}

/// Triangle sum theorem: \
/// Sum of interior angles = 180 \
/// This function determines whether 3 values can be angle lenghts
pub fn sum(a: f32, b: f32, c: f32) -> bool {
    (a + b + c) == 180.0 
}

/// Exterior angle sum theorem \
/// `t_sum` for exterior angles
pub fn ext_sum(a: f32, b: f32, c: f32) -> bool {
    (a + b + c) == 360.0
}