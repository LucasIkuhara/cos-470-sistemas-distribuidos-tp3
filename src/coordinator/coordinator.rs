use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::Write;
use std::sync::{Arc, Mutex};

use chrono::Local;
use rustyline::DefaultEditor;

use crate::channel::Channel;
use request::{Operation, Request};

pub struct Coordinator {
    request_channel: Arc<Channel<(Request, Arc<Channel<()>>)>>,
    release_channel: Arc<Channel<u32>>,
    log_file: Arc<Mutex<File>>,
    metrics: Arc<Mutex<HashMap<u32, u32>>>,
    current_process: Arc<Mutex<()>>,
}

impl Coordinator {
    pub fn new(
        log_filename: &str,
        request_channel: Arc<Channel<(Request, Arc<Channel<()>>)>>,
        release_channel: Arc<Channel<u32>>,
    ) -> Self {
        let log_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(log_filename)
            .unwrap();

        let metrics = Arc::new(Mutex::new(HashMap::<u32, u32>::new()));
        let log_file = Arc::new(Mutex::new(log_file));
        let current_process = Arc::new(Mutex::new(()));

        Self {
            request_channel,
            release_channel,
            log_file,
            metrics,
            current_process,
        }
    }

    pub fn start_consumer(&self) {
        loop {
            let (request, grant_channel) = self.request_channel.recv();

            let mut _current_process = self.current_process.lock().unwrap();
            self.log_request(Operation::Request, request.id);
            self.metrics
                .lock()
                .unwrap()
                .entry(request.id)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            grant_channel.send(());
            self.log_request(Operation::Grant, request.id);

            self.release_channel.recv();
            self.log_request(Operation::Release, request.id);
        }
    }

    pub fn start_cli(&self) {
        let mut rl = DefaultEditor::new().unwrap();

        loop {
            let readline = rl.readline(">> ");

            let command = match readline {
                Ok(line) => line.trim().to_owned(),
                Err(_) => continue,
            };

            match command.as_str() {
                "1" => self.show_queue(),
                "2" => self.show_metrics(),
                "3" => {
                    println!("Terminating the coordinator...");
                    std::process::exit(0);
                }
                _ => {
                    println!("Unknown command: {}", command);
                }
            }
        }
    }

    fn show_queue(&self) {
        println!("Fila de requests:");
        self.request_channel.read_buffer(|queue| {
            for (request, _) in queue.iter() {
                println!(
                    "  Processo: {}, Operação: {:?}",
                    request.id, request.operation
                );
            }
        });
    }

    fn show_metrics(&self) {
        println!("Requests por processo");
        let metrics = self.metrics.lock().unwrap();
        for (id, count) in metrics.iter() {
            println!("  Processo: {}, Requests: {}", id, count);
        }
    }

    fn log_request(&self, operation: Operation, client_id: u32) {
        let timestamp = Local::now();

        let mut log_file = self.log_file.lock().unwrap();

        match operation {
            Operation::Request => {
                writeln!(log_file, "{}: [R] Request - {:?}", timestamp, client_id).unwrap();
            }
            Operation::Grant => {
                writeln!(log_file, "{}: [S] Grant - {:?}", timestamp, client_id).unwrap();
            }
            Operation::Release => {
                writeln!(log_file, "{}: [R] Release - {:?}", timestamp, client_id).unwrap();
            }
        };
    }
}
