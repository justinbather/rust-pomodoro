#[derive(Debug)]

pub struct Config {
    pub intervals: u64,
    pub focus_duration: u64,
    pub break_duration: u64,

}

impl Config {
    pub fn new( args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 4 {
            return Err("Not enough Arguments")
        }

        let intervals = Self::parse_arg(&args[1]).unwrap();
        let break_duration = Self::parse_arg(&args[2]).unwrap();
        let focus_duration = Self::parse_arg(&args[3]).unwrap();

        

        return Ok( Config {focus_duration, break_duration, intervals});
    }

    fn parse_arg(arg: &String) -> Result<u64, Box<dyn std::error::Error>> {
        let arg: u64 = match arg.trim().parse() {
            Ok(arg) => arg,
            Err(e) => {
                println!("Error with argument: {} - {}", arg, e);
                return Err(e.into());
            }
        };

        Ok(arg)
    
        
    }

    
}