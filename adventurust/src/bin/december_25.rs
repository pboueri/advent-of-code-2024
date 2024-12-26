use std::{collections::HashMap, fs, hash::Hash};
use clap::Parser;
use itertools::Itertools;
use log::{self, debug, error, info, Log};


/*
Approach:
- Count keys (n) and locks (m)
- Part 1 approach is:
    - O(n * m) which isn't bad and can be brute forced
*/


#[derive(Parser, Debug)]
struct Key {
    pins: Vec<i32>,
}
#[derive(Parser, Debug)]
struct Lock {
    pins: Vec<i32>,
}


impl Key {
    fn fits_lock(&self, lock: &Lock, max_pins: i32) -> bool {
        self.pins.iter().zip(lock.pins.iter()).all(|(k, l)| k + l < max_pins)
    }
}



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,

}

fn read(file_path: String) -> (Vec<Key>, Vec<Lock>) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    // Keys start with ...... in the first line
    // Locks start with ##### in the first line
    // Then each line either # or ., we accumulate how many per vertical
    for group in contents.split("\n\n"){
        debug!("{}", group);
        let parts = group.split("\n").collect::<Vec<&str>>();
        let mut is_key = false;
        let size = parts[0].len();
        let mut pins = vec![-1;size];
        let skip = "#####";
        for (i, line) in parts.iter().enumerate() {
            if i == 0 {
                match *line {
                    "#####" => is_key = false,
                    "....." => is_key = true,
                    _ => panic!("Unknown character {line}"),
                }
            } 
            for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        pins[j] += 1;
                    }
                }
        }
        if is_key {
            debug!("is_key");
            keys.push(Key { pins:pins.clone() });
            } else {
            debug!("is_lock");
            locks.push(Lock { pins:pins.clone() });
        }
    
    }
   
    (keys, locks)

}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let (keys, locks) = read(file_path);
    let max_pins = keys[0].pins.len() as i32 + 1;
    let mut count = 0;

    debug!("max_pins: {}", max_pins);


    for key in keys {
        debug!("key: {}", key.pins.iter().join(""));
        for lock in &locks {
            debug!("lock: {}", lock.pins.iter().join(""));
            if key.fits_lock(&lock, max_pins) {
                debug!("{} fits {}, {max_pins}", key.pins.iter().join(""), lock.pins.iter().join(""));
                count+= 1;
            }
        }
    }
    info!("Answer 1: {count}");
    
}