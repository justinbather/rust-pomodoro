/*
* TODO: create MVP: start program, set focus_duration for x time and alert os



*/
use std::env;
use std::io;
use std::process;
use std::time;
mod config;

fn main(){

  
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap();

    let current = time::Instant::now();

    let duration: u64 = config.focus_duration / 1000;



    // Prints correctly
    //
    // println!("{:?}", current);

    println!("Stating Pomodoro: {} intervals of {} focus and {} break", config.intervals, config.focus_duration, config.break_duration);
    loop {
        
        if current.elapsed().as_secs() >= duration {
            break;
        }
    }
    
    println!("focus_duration finished after {:?}", current.elapsed().as_secs());
    
    
}



