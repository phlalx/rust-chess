use std::io;
use std::io::Write;

mod position; // TODO keeping this for position to compile
mod color;

fn evaluate_command(command: &String) {
    let mut tokens = command.split_ascii_whitespace();
    let first = tokens.next().expect("No token found");
    println!("first token is {}", first);
    match first.as_ref() {
        "quit" => {
            println!("Bye!");
            std::process::exit(0);
        },
        _ => (),
    }
}

fn main() {
    let mut command = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed flushing stdout");
        io::stdin().read_line(&mut command).expect("Failed to read command");
        evaluate_command(&command);
    }
}
