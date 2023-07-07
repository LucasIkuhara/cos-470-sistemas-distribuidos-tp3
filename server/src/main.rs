use ws::listen;
use std::env::var;
use std::fmt::Display;
use std::sync::{Mutex, Arc};
use std::thread;
use std::io::stdin;


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

    return format!("{}:{}", host, port);
}

fn main() {

    // Request queue
    let mut queue: Vec<Request> = vec![];
    let queue_lock = Arc::new(Mutex::new(queue));
    
    // Get server url
    let address = build_url();
    println!("Starting WS server at ws://{}", address);

    // thread::spawn(move || {
    //     loop {
    //         let command: mut String = stdin();
    //         println!({}, command);
    //     }
    // })

    
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
