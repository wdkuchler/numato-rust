#![allow(unused)]

use std::fs;
use std::fs::OpenOptions;
use std::env;
use std::{thread, time};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let porta = std::env::args().nth(1).expect("no pattern given");
    let command = std::env::args().nth(2).expect("no path given");
    let fifty_millis = time::Duration::from_millis(50);
    let thousand_millis = time::Duration::from_millis(1000);
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&porta)
        .unwrap();
        
    println!("let's wake up interface");
    writeln!(file, "\r");
    
    thread::sleep(fifty_millis);

    if command == "on" {
        println!("let's activate relay");
        writeln!(file, "relay on 0\r");
    } else if command == "off" {
        println!("let's deactivate relay");
        writeln!(file, "relay off 0\r");
    } else if command == "pulse" {
        println!("let's pulse relay");
        writeln!(file, "relay on 0\r");
        thread::sleep(thousand_millis);
        writeln!(file, "relay off 0\r");
    } else {
        println!("Error: invalid command argument {}", command); 
    }
    Ok(())
}
