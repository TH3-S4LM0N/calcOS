use crate::{vga_buffer::{WRITER, BUFFER_HEIGHT}, println};

/// A number that can be returned if the function
/// prints its own formatted output.
pub const ESCAPE_NUMBER: f32 = -23.5675634;

mod parsing;
mod functions;
mod f32_ext;

pub fn process() {
    let line = WRITER.lock().read_line(BUFFER_HEIGHT - 1);

    println!();
    let answer = parsing::parse_maths(line);

    if answer != ESCAPE_NUMBER {
        println!("{answer}");
    }
}