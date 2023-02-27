use crate::vga_buffer::BUFFER_WIDTH;

pub const GARBAGE: ([MathParts; BUFFER_WIDTH], [char; BUFFER_WIDTH]) = (
    [
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
        MathParts::Null,
    ],
    [
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
    ],
);

#[derive(Clone, Copy)]
pub enum MathParts {
    // represents the end of the equation
    Null,

    // numbers
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    // actions
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn parse_maths(maths: [MathParts; BUFFER_WIDTH]) -> f64 {
    let mut simple_equation = [MathParts::Null, MathParts::Null, MathParts::Null];
    let mut iters = 0;
    for mathpart in maths {
        match mathpart {
            MathParts::Null => break,
            MathParts::Zero => simple_equation[iters] = MathParts::Zero,
            MathParts::One => simple_equation[iters] = MathParts::One,
            MathParts::Two => simple_equation[iters] = MathParts::Two,
            MathParts::Three => simple_equation[iters] = MathParts::Three,
            MathParts::Four => simple_equation[iters] = MathParts::Four,
            MathParts::Five => simple_equation[iters] = MathParts::Five,
            MathParts::Six => simple_equation[iters] = MathParts::Six,
            MathParts::Seven => simple_equation[iters] = MathParts::Seven,
            MathParts::Eight => simple_equation[iters] = MathParts::Eight,
            MathParts::Nine => simple_equation[iters] = MathParts::Nine,
            MathParts::Add => simple_equation[iters] = MathParts::Add,
            MathParts::Subtract => simple_equation[iters] = MathParts::Subtract,
            MathParts::Multiply => simple_equation[iters] = MathParts::Multiply,
            MathParts::Divide => simple_equation[iters] = MathParts::Divide,
        }
        iters += 1;
    }

    return do_simple_equation(simple_equation);
}

fn do_simple_equation(eq: [MathParts; 3]) -> f64 {
    let val1 = parse_from_enum(eq[0]);
    let val2 = parse_from_enum(eq[2]);

    match eq[1] {
        MathParts::Add => val1 + val2,
        MathParts::Subtract => val1 - val2,
        MathParts::Multiply => val1 * val2,
        MathParts::Divide => val1 / val2,
        _ => unreachable!(),
    }
}

fn parse_from_enum(num: MathParts) -> f64 {
    match num {
        MathParts::Zero => 0.0,
        MathParts::One => 1.0,
        MathParts::Two => 2.0,
        MathParts::Three => 3.0,
        MathParts::Four => 4.0,
        MathParts::Five => 5.0,
        MathParts::Six => 6.0,
        MathParts::Seven => 7.0,
        MathParts::Eight => 8.0,
        MathParts::Nine => 9.0,
        _ => unreachable!(),
    }
}

pub fn parse_from_char(char: char) -> MathParts {
    match char {
        '0' => MathParts::Zero,
        '1' => MathParts::One,
        '2' => MathParts::Two,
        '3' => MathParts::Three,
        '4' => MathParts::Four,
        '5' => MathParts::Five,
        '6' => MathParts::Six,
        '7' => MathParts::Seven,
        '8' => MathParts::Eight,
        '9' => MathParts::Nine,
        '+' => MathParts::Add,
        '-' => MathParts::Subtract,
        '*' => MathParts::Multiply,
        '/' => MathParts::Divide,
        _ => MathParts::Null,
    }
}
