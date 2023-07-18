use chrono::Local;
use clap::Parser;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    net::TcpStream,
    process, thread,
    time::Duration,
};

use request::{Operation, Request};

#[derive(Parser)]
struct ClientOptions {
    #[clap(short, long, default_value = "8080")]
    server_port: String,

    #[clap(short, long, default_value = "1")]
    repeats: u32,

    #[clap(short, long, default_value = "1")]
    access_duration: u64,

    #[clap(short, long, default_value = "result.log")]
    log: String,
}

fn main() {
    let opts: ClientOptions = ClientOptions::parse();
    let host = format!("localhost:{}", opts.server_port);

    for _ in 0..opts.repeats {
        match TcpStream::connect(host.as_str()) {
            Ok(mut stream) => {
                let pid = std::process::id();

                println!("PID {} - CONNECTED", pid);

                send_request(&mut stream);
                receive_grant(&mut stream);

                access_critical_region(opts.log.as_str(), opts.access_duration);

                send_release(&mut stream);
            }
            Err(e) => {
                println!("Falha ao connectar ao servidor: {}", e);
            }
        }
    }
}

fn access_critical_region(file_path: &str, duration: u64) {
    let timestamp = Local::now();

    {
        let mut log_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Não foi possível abrir o arquivo de log");

        writeln!(log_file, "{} - {}", timestamp, std::process::id())
            .expect("Não foi possível escrever no arquivo de log");
    }

    thread::sleep(Duration::from_secs(duration))
}

fn send_request(stream: &mut TcpStream) {
    let request = Request {
        operation: Operation::Request,
        id: process::id(),
    };

    stream.write(&request.as_bytes()).unwrap();

    println!("PID {} - REQUEST", process::id());
}

fn receive_grant(stream: &mut TcpStream) {
    let mut res_buffer = [0; 5];
    let response = match stream.read_exact(&mut res_buffer) {
        Ok(_) => Request::from(res_buffer),
        Err(e) => {
            eprintln!("PID {} - Falha ao receber pacote: {}", process::id(), e);
            return;
        }
    };

    if response.operation != Operation::Grant && response.id != process::id() {
        eprintln!("PID {} - GRANT inválido", process::id());
        return;
    }

    println!("PID {} - GRANTED", process::id());
}

fn send_release(stream: &mut TcpStream) {
    let release = Request {
        operation: Operation::Release,
        id: process::id(),
    };

    stream.write(&release.as_bytes()).unwrap();

    println!("PID {} - RELEASED", process::id());
}
