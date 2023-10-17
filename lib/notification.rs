use notify_rust::Notification;


pub fn notify (message: &str) -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
    .summary("Pomodoro")
    .body(message)
    .show()?;
    
    Ok(())
}