use super::power::PowerState;
use std::io;

pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

pub fn convert_input(input: &str) -> Result<PowerState, String> {
    match input {
        "Off" => Ok(PowerState::Off),
        "Sleep" => Ok(PowerState::Sleep),
        "Reboot" => Ok(PowerState::Reboot),
        "Shutdown" => Ok(PowerState::Shutdown),
        "Hibernate" => Ok(PowerState::Hibernate),
        _ => Err("invalid input".to_string()),
    }
}
