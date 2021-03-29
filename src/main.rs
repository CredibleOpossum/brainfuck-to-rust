use std::env;
use std::fs;

fn compress_bf(character: String, previous_length: i32, depth: usize) -> String {
    match character.to_string().as_str() {
        ">" => return format!("{}pointer += {};\n", " ".repeat(depth), previous_length),
        "<" => return format!("{}pointer -= {};\n", " ".repeat(depth), previous_length),
        "+" => {
            return format!(
                "{}memory[pointer] = ((memory[pointer] as i16 + {}) % 256) as u8;\n",
                " ".repeat(depth),
                previous_length
            )
        }
        "-" => {
            return format!(
                "{}memory[pointer] = ((memory[pointer] as i16 - {}) % 256) as u8;\n",
                " ".repeat(depth),
                previous_length
            )
        }
        _ => "".to_string(),
    }
}

fn main() {
    let mut bf_program = "".to_owned();

    bf_program.push_str("fn main() {\n");
    bf_program.push_str("    let mut memory: [u8; 30000] = [0; 30000];\n");
    bf_program.push_str("    let mut pointer: usize = 0;\n");

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut depth: usize = 4;
    let mut previous_character: String = " ".to_string();
    let mut previous_length = 1;
    let mut use_read = false;

    for character in contents.chars() {
        if previous_character == character.to_string() {
            previous_length += 1;
        } else {
            bf_program.push_str(&compress_bf(previous_character, previous_length, depth));
            previous_length = 1;
            previous_character = character.to_string();
        }
        match character.to_string().as_str() {
            "." => bf_program.push_str(&format!(
                "{}print!(\"{{}}\", memory[pointer] as char);\n",
                " ".repeat(depth)
            )),
            "," => {
                bf_program.push_str(&format!("{}mem[p] = std::io::stdin().bytes().next().and_then(|result| result.ok()).map(|byte| byte as u8).unwrap();\n", " ".repeat(depth)));
                use_read = true;
            }
            "[" => {
                bf_program.push_str(&format!(
                    "{}while memory[pointer] != 0 {{\n",
                    " ".repeat(depth)
                ));
                depth += 4;
            }
            "]" => {
                depth -= 4;
                bf_program.push_str(&format!("{}}}\n", " ".repeat(depth)));
            }
            _ => (),
        }
    }
    bf_program.push_str("}");

    if use_read {
        println!("use std::io::Read;");
        println!();
    }
    println!("{}", bf_program);
}
