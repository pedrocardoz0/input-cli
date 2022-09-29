#[derive(Debug)]
pub enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
    Invalid,
}

pub fn display_power_action(input: PowerState) {
    match input {
        PowerState::Off => println!("off"),
        PowerState::Sleep => println!("sleeping"),
        PowerState::Reboot => println!("rebooting"),
        PowerState::Shutdown => println!("shutting down"),
        PowerState::Hibernate => println!("hibernating"),
        _ => println!("invalid"),
    };
}
