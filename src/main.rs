use std::io;
use std::io::Write;

fn main() {
    let mut command = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed flushing stdout");
        io::stdin().read_line(&mut command).expect("Failed to read command");
        println!("I got your command {}", command);
    }
}
