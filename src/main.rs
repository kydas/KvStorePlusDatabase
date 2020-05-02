pub mod kv; 

use std::env;



use kv::kv::KvCmd;
use kv::kv::key_value;


fn main() {
    println!("Hello, Welcome to Kyle's KV store! Here, you only get out what you put in!");
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Accepted commands types are: \n kv <args> \n sql <args> (under construction)");
        return;
    }

    let cmd_type = &args[1];

    if cmd_type == "kv" {
        if args.len() <= 3 { panic!("Accepted commands: \n kv put <key> <value> \n kv get <key> \n kv rm <key>" ) }
        let cmd = &args[2];
        if cmd == "get" {
            key_value(KvCmd::GET, &args[3], None)
        } else if cmd == "rm" {
            key_value(KvCmd::RM, &args[3], None)
        } else if cmd == "put" {
            if args.len() <= 4 {panic!("Not enough args for kv put: \n kv put <key> <value>" )};
            key_value(KvCmd::PUT, &args[3], Some(&args[4]));
        } else {
            println!("Accepted commands: \n kv put <key> <value> \n kv get <key> \n kv rm <key>");
        }
    } else if cmd_type == "sql" {
        println!("SQL functionality is under construction") 
    } else {
        println!("Accepted commands types are: \n kv <args> \n sql <args> (under construction)");
    }

}



