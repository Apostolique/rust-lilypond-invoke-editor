#![windows_subsystem = "windows"]

use std::env;
use std::process::Command;
use std::os::windows::process::CommandExt;

// https://stackoverflow.com/a/38455374/1710293
fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lyeditor = match env::var("LYEDITOR") {
        Ok(val) => val,
        Err(_) => String::from("code -g %(file)s:%(line)s:%(column)s"),
    };

    let lyargs : Vec<&str> = args[1].rsplitn(4, ':').collect();

    let file: &str = crop_letters(lyargs[3], "textedit://".chars().count());
    let line: &str = lyargs[2];
    let char_c: &str = lyargs[1];
    let column: &str = lyargs[0];

    lyeditor = lyeditor.replace("%(file)s", file);
    lyeditor = lyeditor.replace("%(line)s", line);
    lyeditor = lyeditor.replace("%(char)s", char_c);
    lyeditor = lyeditor.replace("%(column)s", column);

    Command::new("cmd")
        .arg("/C")
        .arg(lyeditor)
        .creation_flags(0x08000000) // Call command without showing the cmd prompt.
        .spawn()
        .expect("Failed to execute command");
}
