use core::panic;
use std::io::{self, Write};

pub fn get_response(response: &mut String) {
    io::stdout().flush().unwrap();
    response.clear();

    io::stdin()
        .read_line(response)
        .expect("Unable to get response!");

    if response.ends_with('\n') {
        response.pop();
        if response.ends_with('\r') {
            response.pop();
        }
    }
}

pub fn verify_name(name: &String) {
    if name.contains(" ") {
        panic!("Username cannot contains whitespace characters!")
    }
}

pub fn verify_pass(pass: &String) {
    if pass.is_empty() {
        panic!("Password cannot be empty!");
    }
    if *pass == ' '.to_string() {
        panic!("Password cannot be a single whitespace!");
    }
    if pass.len() < 4 {
        panic!("Password must be at least 4 characters long!");
    }

    let mut whitespace_count = 0;

    for x in pass.chars() {
        if x == ' ' {
            whitespace_count += 1;
        }

        if whitespace_count == 2 {
            panic!("Password cannot have more than 1 whitespace!");
        }
    }
}
