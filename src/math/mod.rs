use crate::{
    println,
    math::parsing::{GARBAGE, parse_maths},
    vga_buffer::{BUFFER_HEIGHT, BUFFER_WIDTH, WRITER},
};

mod parsing;
mod functions;

pub fn process() {
    let mut chars: [char; BUFFER_WIDTH] = GARBAGE.1;

    for iters in 0..BUFFER_WIDTH {
        chars[iters] = char::from(
            WRITER.lock().buffer.chars[BUFFER_HEIGHT - 1][iters]
                .read()
                .ascii_character,
        );
    }

    println!();

    // parsing
    let mut maths = GARBAGE.0;

    let mut iters = 0;
    for char in chars {
        maths[iters] = parsing::parse_from_char(char);
        iters += 1;
    }

    let result = parse_maths(maths);
    println!("{result}");
}