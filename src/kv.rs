use std::collections::HashMap;
use ron;
use std::fs::File;
use std::io::prelude::*;


pub enum KvCmd {
    GET,
    PUT,
    RM
}

pub fn key_value(cmd: KvCmd, key: &String, value: Option<&String>) {
    let path = String::from("kv_store.ron");

    let mut kv_store = inst_kv_store(&path);
    match cmd {
        KvCmd::GET =>  {
            get_kv(&key, &kv_store);
        }
        KvCmd::RM => {
            rm_kv(&key, &mut kv_store);
            save_kv_store(&path, &kv_store);
        }
        KvCmd::PUT => {
            put_kv(&key, value.unwrap(), &mut kv_store);
            save_kv_store(&path, &kv_store)
        }
    }
}

fn put_kv(key: &String, value: &String, map: &mut HashMap<String, String>) {
    map.insert(key.clone(), value.clone());
    println!("{} added at {}", value, key);
}

fn get_kv(key: &String, map: &HashMap<String, String>) {
    match map.get(key) {
        Some(i) => println!("Value at {} is: {}", key, i),
        None => println!("No value for {}", key)
    };
}

fn rm_kv(key: &String, map: &mut HashMap<String, String>) {
    match map.remove(key) {
        Some(i) => println!("Value {} at key {}", i, key),
        None => println!("No value at key {}", key)
    };
}
fn inst_kv_store(path: &String) -> HashMap<String, String> {
    let mut file = match File::open(path){
        Err(why) => panic!("couldn't open {}: {}", path, why.to_string()),
        Ok(file) => file
    };
    let file_meta = match file.metadata() {
        Err(why) => panic!("{}", why.to_string()),
        Ok(file_meta) => file_meta 
    };
    if file_meta.len() == 0 {
        return HashMap::new();
    } else {
        let mut file_string = String::new();
        file.read_to_string(&mut file_string).unwrap();
        let kv_store =  match ron::de::from_str::<HashMap<String, String>>(&file_string) {
            Err(why) => panic!("Could not deserialize: {}", why.to_string()),
            Ok(kv_store) => kv_store
        };
        kv_store
    }
}

fn save_kv_store(path: &String, kv_store: &HashMap<String, String>) {
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path, why.to_string()),
        Ok(file) => file,
    };
    let file_string = match ron::ser::to_string(kv_store) {
        Err(_) => panic!("Serialization failed"),
        Ok(file_string) => file_string
    };
    match file.write_all(file_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why.to_string()),
        Ok(_) => ()
    };
}


