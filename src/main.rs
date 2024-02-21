mod database;
mod inputs;
mod users;

use database::*;
use inputs::*;
use users::*;

const ENV_NAME: &str = "Rust Admin Panel";

fn main() {
    let mut user_list = Vec::<User>::new();

    let mut name = String::new();
    let mut pass = String::new();

    loop {
        print!("Welcome to {ENV_NAME}. Who are you? :");

        get_response(&mut name);
        verify_name(&name);

        print!("Give a password for {name} :");

        get_response(&mut pass);
        verify_pass(&pass);

        let user = User {
            name: name,
            pass: pass,
        };

        user_list.push(user.to_owned());

        user_list[0].print_all();

        break;
    }
}
