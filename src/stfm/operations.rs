use std::{fs, fs::File};
use std::env;
use std::io;

pub fn get_all_entries_in_cwd() -> io::Result<Vec<String>> {
    let current_dir = env::current_dir()?;
    let entries = fs::read_dir(current_dir)?;
    let all_entries = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry.file_name().to_str().map(|s| s.to_owned())
        })
        .collect();
    Ok(all_entries)
}

pub fn execute_make(t: &str, name: &str) {
    match t {
        "f" => {
            if File::create_new(name).is_ok() {
                println!("Made file {name}");
            } else {
                println!("Failed to make file {name}");
            }
        }
        "d" => {
            if fs::create_dir_all(name).is_ok() {
                println!("Created directory {name}")
            } else {
                println!("Failed to make directory {name}");
            }
        }
        _ => {
            println!("Expected values f(ile) or d(irectory)")
        }
    }
}

pub fn execute_remove(name: &str) {
    if fs::exists(name).is_ok() {
        if fs::remove_file(name).is_ok() {
            println!("Removed file {name}");
        } else if fs::remove_dir_all(name).is_ok() {
            println!("Removed directory {name}");
        } else {
            println!("Failed to remove {name}");
        }
    }
}
