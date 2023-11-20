use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::Write;
use std::{env, fs};

use log::debug;
use rev_lines::RevLines;

pub(crate) fn names() -> Vec<String> {
    vec![
        String::from("hydrogen"),
        String::from("centauri"),
        String::from("phoenix"),
        String::from("belinda"),
        String::from("haumea"),
        String::from("drago"),
        String::from("mars"),
        String::from("eris"),
    ]
}

pub fn name_thread() -> Result<String, String> {
    let names = names();
    let name_lock_file = get_name_lock_file();
    let file_exists = fs::metadata(name_lock_file.clone());

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(name_lock_file)
        .unwrap();

    if file_exists.is_err() {
        let name = names.get(0).unwrap();
        if let Err(e) = write!(file, "{}", name) {
            eprintln!("failed to lock thread name: {}", e);
        }

        return Ok(name.clone());
    }

    let mut found_last_name = false;
    let last_name = last_locked_name().first().unwrap().clone();

    for name in names {
        if found_last_name {
            if let Err(e) = write!(file, "\n{}", name) {
                eprintln!("failed to lock thread name: {}", e);
            }

            return Ok(name);
        }

        if name == last_name {
            found_last_name = true;
            continue;
        }
    }

    Err(String::from("failed to name thread"))
}

pub(crate) fn remove_name_lock_file() {
    let name_lock_file = get_name_lock_file();
    debug!("name lock file: {}", name_lock_file);
    let _ = fs::remove_file(name_lock_file);
}

fn get_name_lock_file() -> String {
    let mut dir_binding = env::current_dir().unwrap();
    dir_binding.push(".mailer_name_lock");
    dir_binding.to_str().unwrap().to_string()
}

fn last_locked_name() -> Vec<String> {
    let file = File::open(get_name_lock_file()).unwrap();
    let lines: Vec<String> = RevLines::new(BufReader::new(file))
        .map(|l| l.unwrap())
        .collect();
    lines
}
