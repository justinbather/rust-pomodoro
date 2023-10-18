use std::time;
use crate::config;
use std::io;
use std::io::Write;

pub fn start_pomodoro(config: config::Config) {
    for interval in 0..config.intervals {
        println!("Starting interval: {:?}", interval);
        let current = time::Instant::now();
        
        loop {
            let remaining = config.focus_duration - current.elapsed().as_secs();
            print!("\r{}",remaining );
            io::stdout().flush().expect("Flush Failed");
            if current.elapsed().as_secs() >= config.focus_duration / 1000 {
                break;
            }
        }
        println!("\nStarting your break!");
        
        loop {
            let remaining = config.break_duration - current.elapsed().as_secs();
            print!("\r{}",remaining );
            if current.elapsed().as_secs() >= (config.break_duration + config.focus_duration) / 1000 {
                break;
            }
        }

        println!("\nBreak is over!");

    }
}