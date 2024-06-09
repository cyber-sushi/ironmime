# IronMime

IronMime is a Linux daemon to bind touchpad gestures to shell commands.

# Index
- [Installation](https://github.com/cyber-sushi/ironmime/tree/main#installation)
    - [Building from source](https://github.com/cyber-sushi/ironmime/tree/main#building-from-source)
- [Running IronMime](https://github.com/cyber-sushi/ironmime/tree/main#running-ironmime)
- [Configuration](https://github.com/cyber-sushi/ironmime/tree/main#configuration)
- [Troubleshooting](https://github.com/cyber-sushi/ironmime/tree/main#troubleshooting)

## Installation
To install IronMime, you can either download the executable from the [Releases page](https://github.com/cyber-sushi/ironmime/releases) or you can compile it from source using Cargo.
#### Building from source
1. Install `rustup` using your distro's package manager or refer to the [official docs](https://www.rust-lang.org/tools/install) if your distro doesn't ship `rustup`.
2. Run `rustup default stable` which will automatically install Cargo (Rust's package manager) and the Rust toolchain.
3. Git clone and build with:
```
git clone https://github.com/cyber-sushi/ironmime
cd ironmime
cargo build --release
```
Once Cargo is done compiling, you should find IronMime's executable inside `~/ironmime/target/release/`.

## Running IronMime
Make sure that the executable has permissions to run as a program with `chmod +x ironmime` or with Right Click > Properties > "allow executing as program" or something like that, depending on your file manager.

To run IronMime, choose **one** of the following options:
- **Run IronMime as a system service**\
Move the executable into `/usr/bin`.\
Grab `ironmime.service` from this repo and edit the `User=` line with your username.\
Move the file into `/etc/systemd/system`, then run `systemctl daemon-reload`.\
After this, you can start and stop IronMime with `systemctl start/stop ironmime` or you can enable/disable it on startup with `systemctl enable/disable ironmime`. If you change the config files and you want the changes to take place, restart IronMime with `systemctl restart ironmime`.

> [!NOTE]
> When running as a systemd service, IronMime inherits your systemd user environment, not your shell environment (you can see it with `systemctl --user show-environment`).\
If you need to pass env variables to it, do so by adding them to the unit file with `Environment=VARIABLE=value`.

- **Run IronMime as a user service**\
Move the executable into `/usr/bin`.\
Grab `ironmime.service` from this repo and remove the lines that start with `Group` and `User`.\
Move the file into `/etc/systemd/user`, then run `systemctl --user daemon-reload`.\
Add your user to the input group with `sudo usermod -aG input <username>` and reboot.\
After this, you can start and stop IronMime with `systemctl --user start/stop ironmime` or you can enable/disable it on startup with `systemctl --user enable/disable ironmime`. If you change the config files and you want the changes to take place, restart IronMime with `systemctl --user restart ironmime`.

> [!CAUTION]
> Running IronMime as a _user_ service instead of a _system_ service requires your user to be in the input group, which might be a security risk because it allows all applications to read your inputs.

- **Run IronMime without systemd**\
Move the executable into `/usr/bin` or `~/.local/bin` (just make sure it's in `PATH`).\
Add your user to the input group with `sudo usermod -aG input <username>` and reboot.\
Call `ironmime` from a terminal, a script or from your compositor's config file.

> [!CAUTION]
> Running IronMime this way requires your user to be in the input group, which might be a security risk because it allows all applications to read your inputs.

## Configuration
IronMime's config directory defaults to `$HOME/.config/ironmime/ironmime.conf` but it can be changed through the `IRONMIME_CONFIG` environment variable (if you run IronMime as a systemd service, add it directly to the systemd unit).

The syntax to declare a gesture in the config file is the following:
```
<gesture> <direction> <fingers> = <command>
```
There are three available gestures that you can use: `swipe`, `pinch` and `hold`.

| Gesture | Directions | Fingers |
| :--- | :---: | :---: |
| `swipe` | `left` `right` `up` `down `| `3` `4` |
| `pinch` | `in` `out` | `2` `3` `4` |
| `hold` | `on` | `1` `2` `3` `4` |

Examples:
```
# Move one workspace to the right on Hyprland
swipe left 3 = hyprctl dispatch workspace +1
# Move one workspace to the left on Hyprland
swipe down 3 = hyprctl dispatch workspace -1
# Inject Ctrl + Tab
swipe left 4 = wtype -M ctrl -k Tab -m ctrl
# Inject Ctrl + Shift + Tab
swipe right 4 = wtype -M ctrl -M shift -k Tab -m shift -m ctrl
# Close the active window on Hyprland
swipe down 4 = hyprctl dispatch killactive
# Send a notification that prints "ouch"
pinch in 3 = notify-send "ouch"
# Launch Nautilus
pinch out 4 = nautilus
# Take a screenshot
hold on 4 = grim ~/Pictures/Screenshots/$(date +%s).png
# Take a screenshot with area selector
hold on 3 = grim -g "$(slurp)" -t png - | wl-copy -t image/png
```
IronMime can be used in combination with tools like [wtype](https://github.com/atx/wtype) or [xdotool](https://github.com/jordansissel/xdotool) to emulate keypresses and other actions.
> [!WARNING]
> Some gestures may already be bound to certain actions on your environment, e.g. `hold on 1` > Left Click, `pinch out 2` > Zoom in, etc. Look out for possible conflicts.

> [!NOTE]
> If you need to add comments to the config file, put them between the lines (like in the examples) and not at the end of the lines.

## Troubleshooting
**Q**: SELinux prevents IronMime's system service from running, what do I do?\
**A**: Put `ironmime.service` inside `/usr/lib/systemd/system` instead of `/etc/systemd/system`, then run the following commands:
- `sudo semanage fcontext -a -t bin_t "/usr/lib/systemd/system/ironmime.service"`
- `sudo restorecon -v /usr/lib/systemd/system/ironmime.service`
- `sudo semanage fcontext -a -t bin_t "/usr/bin/ironmime"`
- `sudo restorecon -v /usr/bin/ironmime`
