/*
* TODO: create MVP: start program, set timer for x time and alert os



*/

use std::io;
use std::env;
use std::time;
use std::process;

fn main() {
    
    let mut timer = String::new(); 

    println!("Please enter the amount of time in ms");
    
    io::stdin()
        .read_line(&mut timer)
        .expect("Invalid input");

    let timer: u64 = match timer.trim().parse() {
        Ok(time) => time,
        Err(_) => {
            println!("Not an integer");
            return;
            },
    };

    

    let current = time::Instant::now();

    let mut duration: u64 = &timer / 1000;



    // Prints correctly
    //
    println!("{:?}", current);

    loop {
        if (current.elapsed().as_secs() >= duration) {
            break;
        }
    }
    
    println!("Timer finished after {:?}", current.elapsed().as_secs());










}
