#[derive(Debug)]

pub struct Config {
    pub intervals: u64,
    pub focus_duration: u64,
    pub break_duration: u64,

}

impl Config {
    pub fn new(focus_duration: &u64, break_duration: &u64, intervals: &u64) -> Result<Config, &'static str> {

        if intervals < &1 {
            return Err("Not valid inputs");
        };

        let focus_duration = focus_duration.clone();
        let break_duration = break_duration.clone();
        let intervals = intervals.clone();

        

        Ok( Config {focus_duration, break_duration, intervals})
    }

    
}