use crate::info;

#[derive(PartialEq, Debug)]
pub enum MathType {
    Number,
    Action,
    Letter,
    End,
}

pub trait CharExt {
    fn math_type(&self) -> Option<MathType>;
}

impl CharExt for char {
    fn math_type(&self) -> Option<MathType> {
        if *self as u8 == 32 {
            return Some(End);
        }

        use MathType::*;
        match self {
            '0' => Some(Number),
            '1' => Some(Number),
            '2' => Some(Number),
            '3' => Some(Number),
            '4' => Some(Number),
            '5' => Some(Number),
            '6' => Some(Number),
            '7' => Some(Number),
            '8' => Some(Number),
            '9' => Some(Number),

            '+' => Some(Action),
            '-' => Some(Action),
            '*' => Some(Action),
            '/' => Some(Action),
            '^' => Some(Action),

            'a' | 'A' => Some(Letter),
            'b' | 'B' => Some(Letter),
            'c' | 'C' => Some(Letter),
            'd' | 'D' => Some(Letter),
            'e' | 'E' => Some(Letter),
            'f' | 'F' => Some(Letter),
            'g' | 'G' => Some(Letter),
            'h' | 'H' => Some(Letter),
            'm' | 'M' => Some(Letter),
            'p' | 'P' => Some(Letter),
            'q' | 'Q' => Some(Letter),
            'r' | 'R' => Some(Letter),
            's' | 'S' => Some(Letter),
            't' | 'T' => Some(Letter),
            'u' | 'U' => Some(Letter),
            'y' | 'Y' => Some(Letter),

            '_' => Some(Letter),
            _ => None,
        }
    }
}
