use crate::{print, println, vga_buffer::BUFFER_WIDTH};
use arrayvec::ArrayString;
use core::fmt::Write;

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

#[derive(Clone, Copy, PartialEq)]
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

    // letters for function names
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Underscore
}

#[derive(Debug)]
struct MathErr;

impl MathParts {
    
    fn math_type(&self) -> Result<MathTypes, MathErr> {
        use MathParts::*;
        use MathTypes::*;
        match self {
            Null => Err(MathErr),

            Add => Ok(Action),
            Subtract => Ok(Action),
            Multiply => Ok(Action),
            Divide => Ok(Action),

            Zero => Ok(Number),
            One => Ok(Number),
            Two => Ok(Number),
            Three => Ok(Number),
            Four => Ok(Number),
            Five => Ok(Number),
            Six => Ok(Number),
            Seven => Ok(Number),
            Eight => Ok(Number),
            Nine => Ok(Number),

            A => Ok(Letter),
            B => Ok(Letter),
            C => Ok(Letter),
            D => Ok(Letter),
            E => Ok(Letter),
            F => Ok(Letter),
            G => Ok(Letter),
            H => Ok(Letter),
            I => Ok(Letter),
            J => Ok(Letter),
            K => Ok(Letter),
            L => Ok(Letter),
            M => Ok(Letter),
            N => Ok(Letter),
            O => Ok(Letter),
            P => Ok(Letter),
            Q => Ok(Letter),
            R => Ok(Letter),
            S => Ok(Letter),
            T => Ok(Letter),
            U => Ok(Letter),
            V => Ok(Letter),
            W => Ok(Letter),
            X => Ok(Letter),
            Y => Ok(Letter),
            Z => Ok(Letter),
            Underscore => Ok(Letter),
        }
    }

    fn as_f32(&self) -> f32 {
        use MathParts::*;
        match self {
            Zero => 0.0,
            One => 1.0,
            Two => 2.0,
            Three => 3.0,
            Four => 4.0,
            Five => 5.0,
            Six => 6.0,
            Seven => 7.0,
            Eight => 8.0,
            Nine => 9.0,
            _ => unreachable!(),
        }
    }

    fn as_str<'a>(&self) -> Result<&'a str, MathErr> {
        use MathParts::*;
        match self {
            E => Ok("e"),
            H => Ok("h"),
            M => Ok("m"),
            P => Ok("p"),
            Q => Ok("q"),
            R => Ok("r"),
            S => Ok("s"),
            T => Ok("t"),
            Y => Ok("y"),
            Underscore => Ok("_"),
            _ => Err(MathErr)
        }
    }
}

#[derive(PartialEq)]
enum MathTypes {
    Number,
    Action,
    Letter
}

pub fn parse_maths(maths: [MathParts; BUFFER_WIDTH]) -> f32 {
    let mut answer = 0.0;

    let mut iters: usize = 0;

    loop {
        if maths[iters] == MathParts::Null {
            break;
        }
        let mathtype = maths[iters].math_type().unwrap();

        if mathtype == MathTypes::Number {
            print!("\niter {iters} is a number ");
            let (num, new_index) = get_multidigit_num(&maths, iters);
            print!("{num} ");
            answer = num;
            println!("with new index {new_index}");
            iters = new_index - 1;
        } else if mathtype == MathTypes::Action {
            println!("iter {iters} is action: ");
            match maths[iters] {
                MathParts::Add => {
                    iters += 1;
                    let (num, new_index) = get_multidigit_num(&maths, iters);
                    iters = new_index;
                    answer += num;
                },
                MathParts::Subtract => {
                    iters += 1;
                    let (num, new_index) = get_multidigit_num(&maths, iters);
                    iters = new_index;
                    answer -= num;
                },
                MathParts::Multiply => {
                    iters += 1;
                    let  (num, new_index) = get_multidigit_num(&maths, iters);
                    iters = new_index;
                    answer *= num;
                },
                MathParts::Divide => {
                    iters += 1;
                    let (num, new_index) = get_multidigit_num(&maths, iters);
                    iters = new_index;
                    answer /= num;
                },
                _ => unreachable!()
            } 
        } else if mathtype == MathTypes::Letter {

            let mut fn_name = ArrayString::<16>::new();

            loop {
                match fn_name.as_str() {
                    "q_sqrt" => {
                        iters += 1;

                        let (arg, _) = get_multidigit_num(&maths, iters);
                        answer = super::functions::Q_sqrt(arg);
                        break;
                    },
                    "sqrt" => {
                        iters += 1;

                        let (arg, _) = get_multidigit_num(&maths, iters);
                        answer = super::functions::sqrt(arg);
                        break;
                    },
                    "py_thrm" => {
                        iters += 1;

                        let (arg1, new_index) = get_multidigit_num(&maths, iters);
                        iters = new_index;
                        let (arg2, _) = get_multidigit_num(&maths, iters);
                        answer = super::functions::py_thrm(arg1, arg2);
                        break;
                    },
                    _ => (),
                }

                fn_name.push_str(maths[iters].as_str().expect("Reached Null without matching a function"));
                iters += 1;
            }
        }

        iters += 1;
    }

    return answer;
}

/// returns (the number, the ending index)
fn get_multidigit_num(eq: &[MathParts; BUFFER_WIDTH], mut index: usize) -> (f32, usize) {
    let mut num = 0.0;
    loop {
        let part = eq[index];
        if part == MathParts::Null {
            break;
        } else if part.math_type().unwrap() == MathTypes::Number {
            num = (num * 10.0) + part.as_f32();
        } else {
            break;
        }
        index += 1;
    }
    return (num, index);
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

        'e' | 'E' => MathParts::E,
        'h' | 'H' => MathParts::H,
        'm' | 'M' => MathParts::M,
        'p' | 'P' => MathParts::P,
        'q' | 'Q' => MathParts::Q,
        'r' | 'R' => MathParts::R,
        's' | 'S' => MathParts::S,
        't' | 'T' => MathParts::T,
        'y' | 'Y' => MathParts::Y,

        '_' => MathParts::Underscore,
        _ => MathParts::Null
    }
}
