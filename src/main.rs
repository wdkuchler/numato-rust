/*
============================================================================
 Name        : numato-rust
 Author      : Willy Kuchler
 Version     : 1.0
 Copyright   : Numato Systems Pvt. Ltd.
 Product     : Numato Lab 1 Channel USB Powered Relay Module
 Device      : idVendor=2a19, idProduct=0c05
 Caveats     : This program was written for training in the Rust language      
============================================================================
*/

#![allow(unused)]

use std::fs::File;
use std::fs::OpenOptions;
use std::env;
use std::{thread, time};
use std::io::{prelude::*, Result};

fn wakeup(fd: &mut File) -> std::io::Result<()> {
    println!("let's wake up interface");
    write!(fd, "{}", '\r');
    thread::sleep(time::Duration::from_millis(50));
    Ok(())
}

fn activate(fd: &mut File) -> std::io::Result<()> {
    println!("let's activate relay");
    write!(fd, "relay on 0\r");
    Ok(())
}

fn deactivate(fd: &mut File) -> std::io::Result<()> {
    println!("let's deactivate relay");
    write!(fd, "relay off 0\r");
    Ok(())
}

fn pulse(fd: &mut File) -> std::io::Result<()> {
    let thousand_millis = time::Duration::from_millis(1000);

    println!("let's pulse relay");
    write!(fd, "relay on 0\r");
    thread::sleep(time::Duration::from_millis(1000));
    write!(fd, "relay off 0\r");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let porta = std::env::args().nth(1).expect("no pattern given");
    let command = std::env::args().nth(2).expect("no path given");
    let mut fd = OpenOptions::new().create(true).write(true).append(true).open(&porta).unwrap();    
    
    println!("USB Relay Module Controller - USBPOWRL002");       
    
    wakeup(&mut fd);
 
    match command.to_owned().to_lowercase() .as_str() {
        "on" => activate(&mut fd),
        "off" => deactivate(&mut fd),
        "pulse" => pulse(&mut fd),
        _ => panic!(),
        };
    Ok(())
}
