use std::{collections::HashMap, process::{Command, exit}, str::FromStr, env, fs};


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s {
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Scale {
    In,
    Out,
}

impl FromStr for Scale {
    type Err = String;
    fn from_str(s: &str) -> Result<Scale, Self::Err> {
        match s {
            "in" => Ok(Scale::In),
            "out" => Ok(Scale::Out),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GestureType {
    Swipe(Direction),
    Pinch(Scale),
    Hold,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Gesture {
    pub gesture_type: GestureType,
    pub fingers: i32,
}

pub fn parse_config() -> (HashMap<Gesture, String>, bool) {
    let mut config_map: HashMap<Gesture, String> = HashMap::default();
    let systemd = set_environment();
    let path = if let Ok(var) = env::var("IRONMIME_CONFIG") {
        var
    } else {
        let home = env::var("HOME").unwrap();
        format!("{}/.config/ironmime/ironmime.conf", home)
    };
    println!("Reading config from {}.", path);
    let config = fs::read_to_string(path).expect("Config file not found.");
    for line in config.split("\n") {
        if let Some((gesture, command)) = line.split_once("=") {
            let parameters = gesture.split_whitespace().collect::<Vec<&str>>();
            if parameters[0] == "swipe" {
                let gesture = Gesture {
                    gesture_type: GestureType::Swipe(Direction::from_str(parameters[1]).unwrap()),
                    fingers: parameters[2].parse::<i32>().expect("Finger number can only be an int.")
                };
                let command = command.trim();
                config_map.insert(gesture, command.to_string());
            } else if parameters[0] == "pinch" {
                let gesture = Gesture {
                    gesture_type: GestureType::Pinch(Scale::from_str(parameters[1]).unwrap()),
                    fingers: parameters[2].parse::<i32>().expect("Finger number can only be an int.")
                };
                let command = command.trim();
                config_map.insert(gesture, command.to_string());
            } else if parameters[0] == "hold" {
                let gesture = Gesture {
                    gesture_type: GestureType::Hold,
                    fingers: parameters[2].parse::<i32>().expect("Finger number can only be an int.")
                };
                let command = command.trim();
                config_map.insert(gesture, command.to_string());
            }
        }
    }
    (config_map, systemd)
}

fn set_environment() -> bool {
    match env::var("DBUS_SESSION_BUS_ADDRESS") {
        Ok(_) => {
            println!("Running with user privileges.");
            let groups = Command::new("groups").output().unwrap();
            if !std::str::from_utf8(&groups.stdout.as_slice()).unwrap().contains("input") {
                println!("IronMime has no access to libinput. Run IronMime as a system service or add your user to the input group.\n\
                        Exiting IronMime.");
                exit(0)
            }
            copy_variables()
        },
        Err(_) => {
            println!("Running as a system service.");
            let uid = Command::new("sh").arg("-c").arg("id -u").output().unwrap();
            let uid_number = std::str::from_utf8(uid.stdout.as_slice()).unwrap().trim();
            if uid_number != "0" {
                let bus_address = format!("unix:path=/run/user/{}/bus", uid_number);
                env::set_var("DBUS_SESSION_BUS_ADDRESS", bus_address);
                copy_variables()
            } else {
                println!("Warning: unable to inherit user environment.\n\
                        Make sure that your systemd unit is running with the 'User=<username>' parameter.\n\
                        Exiting IronMime.");
                exit(0)
            }
        },
    }
}

fn copy_variables() -> bool {
    if let Ok(command) = Command::new("sh").arg("-c").arg("systemctl --user show-environment").output() {
        let vars = std::str::from_utf8(command.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        for var in vars {
            if let Some((variable, value)) = var.split_once("=") {
            	if let Err(env::VarError::NotPresent) = env::var(variable) {
                	env::set_var(variable, value);
                } else if variable == "PATH" {
                    env::set_var("PATH", format!("{}:{}", value, env::var("PATH").unwrap()));
                }
            }
        }
        if let (Err(env::VarError::NotPresent), Ok(_)) = (env::var("XDG_SESSION_TYPE"), env::var("WAYLAND_DISPLAY")) {
            env::set_var("XDG_SESSION_TYPE", "wayland")
        }
        true
    } else {
        false
    }
}
