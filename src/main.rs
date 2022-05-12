use owo_colors::OwoColorize;
use std::{
    env,
    path::Path,
    process::{exit, Command},
    str,
    str::FromStr,
};

mod mac;
use mac::*;

const ROOT_CHECK: bool = true;
const SET_LOCAL_BIT: bool = true; //specifies whether to set mac as locally administered (Reccomended to leave enabled)
const SYSFS_PATH: &str = "/sys";
const HELP_MSG: &str = "github.com/1Michael23

    Usage: Macchanger-rs <option> <interface>

    Options:
        -h          Prints this help message
        -r          Sets a random Mac Address
        -m <mac>    Sets a specific Mac Address
";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help()
    } else {
        match args[1].as_str() {
            "-h" => help(),

            "-r" => {
                if args.len() == 3 {
                    set_mac(&args[2], Mac::new_random());
                } else {
                    help()
                }
            }

            "-m" => {
                if args.len() == 4 {
                    match Mac::from_str(&args[2]) {
                        Ok(mac) => set_mac(&args[3], mac),
                        Err(e) => {
                            println!("{} Invalid Mac Address, ({})", "Error:".bright_red(), e)
                        }
                    }
                } else {
                    help()
                }
            }
            _ => help(),
        }
    }
}

fn tests(verbose: bool) {
    let user = env::var("USER").unwrap();
    if user != "root" && ROOT_CHECK {
        println!(
            "{} Running as user: {}, Program must be run as root.",
            "Error:".bright_red(),
            user.bright_red()
        );
        exit(1);
    } else if verbose {
        println!(
            "{} Running as user: {}",
            "Ok:".bright_green(),
            user.bright_green()
        );
    }

    let sysfs_path = Path::new(SYSFS_PATH);
    if !sysfs_path.exists() {
        println!(
            "{} Sysfs directory not found at {}",
            "Error".bright_red(),
            SYSFS_PATH.bright_red()
        )
    } else if verbose {
        println!("{} Sysfs path found.", "Ok:".bright_green());
    }

    let ip_output = Command::new("ip").arg("-V").output();
    match ip_output {
        Ok(_) => {
            if verbose {
                println!("{} Ip command found.", "Ok:".bright_green());
            }
        }
        Err(e) => {
            println!("{} Ip command not found, ({})", "Error:".bright_red(), e);
        }
    }
}

fn set_mac(interface: &str, mac: Mac) {
    tests(false);

    let downoutput = Command::new("ip")
        .args(["link", "set", "dev", interface, "down"])
        .output()
        .unwrap();

    match downoutput.status.success() {
        true => println!(
            "{} Stopped interface: {}",
            "Ok:".bright_green(),
            interface.bold()
        ),
        false => {
            println!(
                "{} Unable to stop inerface: {}, ({})",
                "Error:".bright_red(),
                interface.bold(),
                str::from_utf8(&downoutput.stderr)
                    .unwrap()
                    .trim_end_matches('\n')
                    .bright_red()
            );
            exit(downoutput.status.code().unwrap_or(1))
        }
    }

    let changeoutput = Command::new("ip")
        .args([
            "link",
            "set",
            "dev",
            interface,
            "address",
            mac.to_string().as_str(),
        ])
        .output();

    match changeoutput {
        Ok(output) => match output.status.success() {
            true => {
                println!(
                    "{} Set {} Mac to [{}]",
                    "Ok:".bright_green(),
                    interface.bold(),
                    mac.to_string().bright_red().bold()
                );
            }
            false => {
                println!(
                    "{} Failed to set Mac address, ({}) trying to restore interface",
                    "Error:".bright_red(),
                    str::from_utf8(&output.stderr)
                        .unwrap()
                        .trim_end_matches('\n')
                        .bright_red()
                );
            }
        },
        Err(e) => println!(
            "{} Failed to set Mac address, ({}) trying to restore interface",
            "Error:".bright_red(),
            e
        ),
    }

    let upoutput = Command::new("ip")
        .args(["link", "set", "dev", interface, "up"])
        .output();

    match upoutput {
        Ok(output) => {
            match output.status.success() {
                true => {
                    println!(
                        "{} Restored interface: {}",
                        "Ok:".bright_green(),
                        interface.bold()
                    );
                }
                false => {
                    println!("{} Failed to restore interface {}, ({}) bring up manually or reboot to fix", "Error:".bright_red(), interface, str::from_utf8(&output.stderr).unwrap().trim_end_matches('\n').bright_red());
                    exit(output.status.code().unwrap_or(1))
                }
            }
        }
        Err(e) => {
            println!(
                "{} Failed to restore interface {}, ({}) bring up manually or reboot to fix",
                "Error:".bright_red(),
                interface.bold(),
                e.bright_red()
            );
            exit(1)
        }
    }
}

fn help() {
    let title = "Macchanger-rs";

    println!("{}\n{}", title.bold().bright_green(), HELP_MSG);

    exit(1);
}
