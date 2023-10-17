/*
* TODO: create MVP: start program, set focus_duration for x time and alert os



*/

use std::io;
use std::process;
use std::time;
mod config;

fn main(){

  
    
    let mut focus_duration = String::new(); 

    let mut num_intervals = String::new();
    let mut break_duration = String::new();

    println!("How long do you want your focus sessions?");
    
    io::stdin()
        .read_line(&mut focus_duration)
        .expect("Invalid input");

    let focus_duration: u64 = match focus_duration.trim().parse() {
        Ok(time) => time,
        Err(_) => {
            println!("Not an integer");
            process::exit(1)
            },
    };

    println!("How long should your breaks be?");

    io::stdin()
    .read_line(&mut break_duration)
    .expect("Invalid input for break duration");

    let break_duration: u64 = match break_duration.trim().parse() {
        Ok(time) => time,
        Err(_) => {
            println!("Not an integer for break duration");
            process::exit(1)
        }
    };

    println!("How many intervals would you like? Min 1");

    io::stdin()
    .read_line(&mut num_intervals)
    .expect("Invalid entry for intervals");

    let num_intervals: u64 = match num_intervals.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid interval");
            process::exit(1)
        },
    };


    let config = config::Config::new(&focus_duration, &break_duration, &num_intervals).unwrap();

    println!("{:#?}", config);
    



    let current = time::Instant::now();

    let duration: u64 = &focus_duration / 1000;



    // Prints correctly
    //
    println!("{:?}", current);

    println!("Stating Pomodoro: {} intervals of {} focus and {} break", num_intervals, focus_duration, break_duration);
    loop {
        
        if current.elapsed().as_secs() >= duration {
            break;
        }
    }
    
    println!("focus_duration finished after {:?}", current.elapsed().as_secs());
    
    
}



