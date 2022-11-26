#![allow(unused)]

use std::fs;
use std::io::prelude::*;
use std::env;
use std::{thread, time};

fn main() {
    let porta = std::env::args().nth(1).expect("no pattern given");
    let command = std::env::args().nth(2).expect("no path given");
    let fifty_millis = time::Duration::from_millis(50);
    let thousand_millis = time::Duration::from_millis(1000);
    
    println!("let's wake up interface");
    fs::write(&porta, b"\r");
    
    thread::sleep(fifty_millis);

    if command == "on" {
        println!("let's activate relay");
        fs::write(&porta, b"relay on 0\r");
    } else if command == "off" {
        println!("let's deactivate relay");
        fs::write(&porta, b"relay off 0\r");
    } else if command == "pulse" {
        println!("let's pulse relay");
        fs::write(&porta, b"relay on 0\r");
        thread::sleep(thousand_millis);
        fs::write(&porta, b"relay off 0\r");
    } else {
        println!("Error: invalid command argument {}", command); 
    }
}
