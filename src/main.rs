use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn find_in_path(cmd: &str) -> Option<String> {
    let path_var = match env::var("PATH") {
        Ok(value) => value,
        Err(_) => return None,
    };

    for dir in path_var.split(':') {
        let full_path = Path::new(dir).join(cmd);

        let metadata = match fs::metadata(&full_path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if !metadata.is_file() {
            continue;
        }

        let mode = metadata.permissions().mode();
        if mode & 0o111 != 0 {
            return Some(full_path.display().to_string());
        }
    }

    None
}

fn external_exucutable(cmd: &str) {
    let path = find_in_path(cmd);
    match path {
        Some(full_path) => {
            let run = Command::new(full_path).status();
            let count = cmd.split_whitespace().count();
            println!("Program was passed with {} arguments", count - 1);
            println!("{:?}", run);
        }
        None => {
            println!("This program doesnot exist")
        }
    }
}

fn main() {
    let valid_commands = ["echo", "exit", "type"];
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();

        let cmnd = command.trim();

        if cmnd == "exit" {
            break;
        } else if cmnd.starts_with("echo") {
            print!("{}", &command[5..]);
        } else if cmnd.starts_with("type") {
            let Some(arg) = cmnd.strip_prefix("type ") else {
                // .strip_prefix returns a <option &str>
                continue;
            };
            if valid_commands.contains(&arg) {
                println!("{} is a shell builtin", arg);
            } else if let Some(full_path) = find_in_path(arg) {
                println!("{} is {}", arg, full_path);
            } else {
                println!("{}: not found", arg);
            }
        } else if cmnd.starts_with("./") {
            external_exucutable(cmnd);
        } else if cmnd.starts_with("pwd") {
            println!("{}", env::current_dir().unwrap().display())
        } else {
            let first_word = cmnd.split_whitespace().next().unwrap_or(cmnd);
            println!("{}: command not found", first_word);
        }
    }
}
