use crate::stfm::operations::*;

pub fn parse_input(buffer: &mut String) -> bool {
    let mut tokens = buffer.split_whitespace();
    while let Some(token) = tokens.next() {
        match token {
            "q" => {
                return false;
            }
            "m" => {
                if let (Some(t), Some(name)) = (tokens.next(), tokens.next()) {
                    execute_make(t, name);
                } else {
                    println!("Expected 2 arguments after m");
                }
            }
            "r" => {
                if let Some(name) = tokens.next() {
                    execute_remove(name);
                } else {
                    println!("Expected path after d");
                }
            }
            _ => {
                println!("Unexpected token");
            }
        }
    }
    return true;
}
