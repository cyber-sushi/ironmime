use std::{collections::HashMap, process::{Command, Stdio}};
use fork::{fork, Fork, setsid};
use crate::parser::Gesture;


pub fn run_command(gesture: Gesture, config: &(HashMap<Gesture, String>, bool), user: &String) {
    if config.1 {
        if let Some(command) = config.0.get(&gesture) {
            Command::new("sh")
                .arg("-c")
                .arg(format!("systemd-run --user -M {}@ {}", user, command))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
        }
    } else {
        if let Some(command) = config.0.get(&gesture) {
            match fork() {
                Ok(Fork::Child) => {
                    match fork() {
                        Ok(Fork::Child) => {
                            setsid().unwrap();
                            Command::new("sh")
                                .args(["-c", command])
                                .stdin(Stdio::null())
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .spawn()
                                .unwrap();
                            std::process::exit(0);
                        }
                        Ok(Fork::Parent(_)) => std::process::exit(0),
                        Err(_) => std::process::exit(1),
                    }
                }
                Ok(Fork::Parent(_)) => (),
                Err(_) => std::process::exit(1),
            }
        }
    }
}
