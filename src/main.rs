/*
* TODO: create MVP: start program, set timer for x time and alert os



*/

use std::io;
use std::process;
use std::time;

use notify_rust::Notification;
use notify_rust::NotificationHandle;

fn main(){
    
    let mut timer = String::new(); 

    println!("Please enter the amount of time in ms");
    
    io::stdin()
        .read_line(&mut timer)
        .expect("Invalid input");

    let timer: u64 = match timer.trim().parse() {
        Ok(time) => time,
        Err(_) => {
            println!("Not an integer");
            process::exit(1)
            },
    };

    

    let current = time::Instant::now();

    let duration: u64 = &timer / 1000;



    // Prints correctly
    //
    println!("{:?}", current);
    let notification_result = send_notification();
    match notification_result {
        Ok(_) => println!("Notification success: {:?}", notification_result),
        Err(err) => println!("Eror sending notification: {}", err)
    }

    loop {
        if current.elapsed().as_secs() >= duration {
            break;
        }
    }
    
    println!("Timer finished after {:?}", current.elapsed().as_secs());
    

    // let bundle = get_bundle_identifier_or_default("");
    // println!("{:?}", &bundle);
    // match set_application(&bundle) {
    //     Ok(_) => {
    //         println!("Application set success:");

    //         let notification = send_notification(
    //             "NOW",
    //             Some("Subtitle"),
    //             "Without subtitle",
    //             Some(Notification::new().sound("Blow"))
    //         );

    //         match notification {
    //             Ok(_) => {
    //                 println!("Notification sent successfully")
    //             },
    //             Err(e) => println!("Error sending notification: {}", e)
    //         };

    //     },
    //     Err(e) => println!("Error setting application: {}", e)
    // }
    
}

fn send_notification() -> Result<(), Box<dyn std::error::Error>> {
    let notification = Notification::new()
    .summary("Safari Crashed")
    .subtitle("subtitle")
    
    .body("Just kidding, this is just the notify_rust example.")
    .show()?;

    println!("{:?}", notification);

    
    Ok(())
}


