/*
TODO: create MVP: start program, set focus_duration for x time and alert os
TODO: CLI Timer
TODO: Timer Logic



*/

/*
* Usage: cargo run -- {intervals} {focus_duration} {break_duration} 
* Intervals must be greater than 1
* Focus Duration & Break Duration must be longer than 1000ms
*/
use std::env;
// use std::io;
// use std::process;

mod config;
mod timer;

fn main(){

  
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap();

    // let current = time::Instant::now();

    

    


    println!("Stating Pomodoro: {} intervals of {} focus and {} break", config.intervals, config.focus_duration, config.break_duration);

    timer::start_pomodoro(config)

    // while remaining_intervals > 0 {

        
        
    //     if current.elapsed().as_secs() >= config.focus_duration {
    //         break;
    //     }
    // }

    
    
    // println!("focus_duration finished after {:?}", current.elapsed().as_secs());
    
    
}



