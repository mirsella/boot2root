use std::fs::{read_to_string, File};
use turtle_graphics::{Canvas, Position, Turtle};

fn main() {
    let mut turtle = Canvas::new();
    let instructions = read_to_string("turtle").expect("coudn't find turtle file");
    let mut char_count = 1;
    for instruction in instructions.lines() {
        if instruction.is_empty() {
            let mut pos = Position::origin();
            pos.0 += char_count as f32 * 300f32;
            turtle.goto(pos);
            char_count += 1;
            continue;
        }
        let value: f32 = match instruction.split_whitespace().nth_back(1).unwrap().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if instruction.starts_with("Avance") {
            turtle.forward(value);
        } else if instruction.starts_with("Recule") {
            turtle.backward(value);
        } else if instruction.starts_with("Tourne droite") {
            turtle.right(value);
        } else if instruction.starts_with("Tourne gauche") {
            turtle.left(value);
        } else {
            dbg!(instruction);
        }
    }
    turtle
        .save_svg(&mut File::create("out.svg").unwrap())
        .unwrap();
}
