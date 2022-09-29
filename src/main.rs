use local::{
    input::{convert_input, get_input},
    power::{display_power_action, PowerState},
};

fn main() {
    let _input = get_input();

    let converted_input = match _input {
        Ok(result) => Ok(match convert_input(&result) {
            Ok(_return) => _return,
            Err(_e) => PowerState::Invalid,
        }),
        Err(e) => Err(e),
    };

    match converted_input {
        Ok(result) => display_power_action(result),
        Err(e) => println!("{:?}", e),
    }
}
