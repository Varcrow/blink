use std::path::PathBuf;
use std::{fs, fs::File};

use color_eyre::eyre::Ok;

pub fn get_entries_in_path(path: &PathBuf) -> Vec<String> {
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
