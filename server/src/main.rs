use ws::listen;
use std::env::var;
use std::process::exit;
use std::sync::{Mutex, Arc};
use std::thread;
use std::io::{stdin, stdout, Write};


/// All possible message types in the system.
#[derive(Debug)]
enum MessageType {
    Request,
    Grant,
    Release
}

/// Represents a request received from a client.
struct Request {
    remote_process: String,
    from_socket: u32,
    message_type: MessageType
}

impl Request {
    
    /// Create a new Request from a message.
    fn from_message(input: &ws::Message, socket_id: u32) -> Request {

        let text: &str = input.as_text().expect("Failed to parse received message.");
        let values: Vec<&str> = text.split("|").collect();

        Request {
            remote_process: values[0].to_string(),
            from_socket: socket_id,
            message_type: if values[1] == "REQ" 
                {MessageType::Request} else {MessageType::Release}
        }
    }
}

/// Implement Display to make Request printable
impl std::fmt::Display for Request {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t- [from: {}, type: {:?}]", self.remote_process, self.message_type)
    }
}

/// Compute the server URL based on the HOST and PORT environment variables.
/// If HOST is unavailable, default to "0.0.0.0",
/// and if PORT is unavailable, default to 5000.
/// The resulting URL will not have the ws:// prefix.
fn build_url() -> String {

    let host: String = match var("HOST") {
        Ok(val) => val,
        Err(_) => String::from("0.0.0.0")
    };

    let port: String = match var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("5000")
    };

    return format!("{}:{}", host, "1234");
}

/// Display all possible CLI commands.
fn cli_help() {
    println!("Available commands:\n\t0. Display this message.\n\t1. Show current queue\n\t2. Metrics by client\n\t3. Terminate execution\n");
}

/// Display a command-line menu to the user, and handle input appropriately.
fn handle_cli_input(queue: Arc<Mutex<Vec<Request>>>) {

    cli_help();
    loop {

        // Read command
        print!(">> ");
        stdout().flush().unwrap();
        let mut buffer: String = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read command.");
        let command = buffer.trim();

        // Handle input
        match command {

            "0" => cli_help(),
            "1" => {
                let q = queue.lock().unwrap();
        
                println!("(HEAD)");
                for req in q.iter() {
                    println!("{}", req);
                }
            },
            "2" => println!("Not implemented"),
            "3" => exit(0),
            _ => println!("Invalid command. To list available commands, type 'help'.")
        }
    }

}

fn main() {

    // Request queue
    let mut queue: Vec<Request> = vec![];
    let queue_lock: Arc<Mutex<Vec<Request>>> = Arc::new(Mutex::new(queue));

    // Get server url
    let address = build_url();
    println!("Starting WS server at ws://{}", address);

    // Start CLI in another thread
    thread::spawn(move || handle_cli_input(queue_lock.clone()));

    
    // Create WS server    
    listen(address, |out| {

        move |msg: ws::Message| {
            println!("{}", out.connection_id());
            let req = Request::from_message(&msg, 12);
            println!("{}", req);
            // send()
            out.send(msg)
       }
    }).expect("Failed to create WS server. Aborting.");
}
