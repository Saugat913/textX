mod editor;
use colored::Colorize;

use std::{env::args, process::exit};

fn display_usage(app_name: &String) {
    println!("{} <filename>", app_name);
    exit(1);
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("{} Filename should be provided !", "ERROR".red());
        display_usage(&args[0]);
    }

    let editor = editor::Editor::default();
    editor.run();
   
}
