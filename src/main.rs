#![allow(unused)]

use std::borrow::Borrow;
use std::fmt::Error;
use std::fs;
use std::fs::OpenOptions;
use std::env;
use std::{thread, time};
use std::io::{prelude::*, Result};

fn wakeup(porta: String) -> std::io::Result<()> {
    let fifty_millis = time::Duration::from_millis(50);

    let mut file = OpenOptions::new().write(true).append(true).open(&porta).unwrap();

    println!("let's wake up interface");
    write!(file, "{}", '\r');
    thread::sleep(fifty_millis);
    Ok(())
}

fn activate(porta: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(&porta).unwrap();

    println!("let's activate relay");
    write!(file, "relay on 0\r");
    Ok(())
}

fn deactivate(porta: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(&porta).unwrap();

    println!("let's deactivate relay");
    write!(file, "relay off 0\r");
    Ok(())
}

fn pulse(porta: String) -> std::io::Result<()> {
    let thousand_millis = time::Duration::from_millis(1000);

    let mut file = OpenOptions::new().write(true).append(true).open(&porta).unwrap();

    println!("let's pulse relay");
    write!(file, "relay on 0\r");
    thread::sleep(thousand_millis);
    write!(file, "relay off 0\r");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let porta = std::env::args().nth(1).expect("no pattern given");
    let command = std::env::args().nth(2).expect("no path given");
       
    wakeup(porta.clone());
 
    match command.as_str() {
        "on" => activate(porta.clone()),
        "off" => deactivate(porta.clone()),
        "pulse" => pulse(porta.clone()),
        _ => panic!(),
        };
    Ok(())
}
