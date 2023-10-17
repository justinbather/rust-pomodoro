

use std::time;

use crate::config;

pub fn start_pomodoro(config: config::Config) {
    for interval in 0..config.intervals {
        println!("Starting interval: {:?}", interval);
        let current = time::Instant::now();
        loop {
            
            if current.elapsed().as_secs() >= config.focus_duration / 1000 {
                break;
            }
        }
        println!("Starting your break!");
        
        loop {
            if current.elapsed().as_secs() >= (config.break_duration + config.focus_duration) / 1000 {
                break;
            }
        }

        println!("Break is over!");

    }
}