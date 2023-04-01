use super::ESCAPE_NUMBER;
use crate::vga_buffer::BUFFER_WIDTH;

pub mod sqrt;
pub mod triangles;

const WIDTH: usize = 5;

#[derive(Clone, Copy)]
pub struct Arguments {
    args: [f32; WIDTH],
}

impl Arguments {
    pub fn new(args: &[char; BUFFER_WIDTH], mut index: usize) -> (Self, usize) {
        let mut new: Self = Self {
            args: [
                ESCAPE_NUMBER,
                ESCAPE_NUMBER,
                ESCAPE_NUMBER,
                ESCAPE_NUMBER,
                ESCAPE_NUMBER,
            ],
        };

        let get_numarg = |args: &[char; BUFFER_WIDTH], mut index: usize| -> (f32, usize, bool) {
            let mut num: f32 = 0.0;

            if args[index] == ')' {
                unreachable!()
            }

            loop {
                let part = args[index];
                let partu8 = part as u8;
                const BANNED: (u8, u8) = (',' as u8, ')' as u8);

                if partu8 == BANNED.0 {
                    return (num, index + 1, false);
                }
                if partu8 == BANNED.1 {
                    return (num, index + 1, true);
                }

                if let Some(dig) = part.to_digit(10) {
                    num = (num * 10.0) + dig as f32;
                } else {
                    todo!("Handle nested equations");
                }
                index += 1;
            }
        };

        let mut fin: bool;
        for iter in 0..5 {
            (new.args[iter], index, fin) = get_numarg(&args, index);
            if fin {
                break;
            }
        }

        return (new, index);
    }

    pub fn arg(&self, index: usize) -> f32 {
        if index > WIDTH {
            todo!("Error handling");
        }

        return self.args[index];
    }
}
