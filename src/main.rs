use std::{env::{args, set_current_dir}, process::Command};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Please provide either --disable, --mid or --high as an argument");
        return;
    }

    let mode: &String = &args[1];

    if mode == "--disable" { 
        change_light_level(0);
    } else if mode == "--mid" { 
        change_light_level(1) 
    } else if mode == "--high" { 
        change_light_level(2) 
    } else {
        eprintln!("The option {} is not supported!", mode);
    }
}

fn change_light_level(level: i8) {
    set_current_dir("/sys/class/leds/tpacpi::kbd_backlight/")
        .expect("Failed to change directory");

    let command: String = format!("echo {} | sudo tee brightness", level);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Command executed successfully:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
