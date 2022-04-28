use std::{env, process::{exit, Command}};

use owo_colors::OwoColorize;
use mac::*;

mod mac;

const ROOT_CHECK: bool = true;


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help()
    }else{
        match args[1].as_str(){
            "-h"|"-H"|"--help" => help(),

            "-r"|"-R"|"--random" => 
                if args.len() == 3 {
                    set_mac(&args[2], Mac::new_random());
                }else{help()},

            
    
            _ => {println!("Invalid option: {}\n", args[1].bright_red());help()}
        }
    }
    {   //Check for root user in limited scope
        let user = env::var("USER").unwrap();
        if user != "root" && ROOT_CHECK == true{
            println!("Running as user: {}, Program must be run as root.", user.bright_red());
            exit(1);
        }else{
            println!("Running as user: {}", user.bright_green());
        }
    }
}

fn set_mac(interface: &str, mac: Mac){

    

let _downoutput = Command::new("ip").args(["link","set", "dev", interface, "down"]).output().unwrap();
let _changeoutput = Command::new("ip").args(["link","set", "dev", interface, "address", mac.to_string().as_str()]).output().unwrap();
let _upoutput = Command::new("ip").args(["link","set", "dev", interface, "up"]).output().unwrap();

}

fn help(){

    let title = "Macchanger-rs";
    let text = "github.com/1Michael23/macchanger-rs\n\n  Usage:\n   macchanger <options> <?interface>\n\n  Options:\n     -r         Generates a random mac address\n     -m <Mac>   Set Mac to a specific address\n";

    println!("{}\n{}",title.bold().bright_green(), text);

    exit(1);

}
