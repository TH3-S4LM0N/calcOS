use crate::{println, math::ESCAPE_NUMBER};

/// Takes 3 angle measures and determines if a triangle can be made;
pub fn valid_angles(args: super::Arguments) -> f32 {
    let side1 = args.arg(0);
    let side2 = args.arg(1);
    let side3 = args.arg(2);

    let answer = side1 + side2 + side3 == 180.0;

    println!("{answer}");

    return ESCAPE_NUMBER;
}

pub fn valid_ext_angles(args: super::Arguments) -> f32 {
    let side1 = args.arg(0);
    let side2 = args.arg(1);
    let side3 = args.arg(2);

    let answer = side1 + side2 + side3 == 360.0;

    println!("{answer}");

    return ESCAPE_NUMBER;
}