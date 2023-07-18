use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use crate::channel::Channel;
use request::{Operation, Request};

pub struct Server {
    listener: TcpListener,
    request_channel: Arc<Channel<(Request, Arc<Channel<()>>)>>,
    release_channel: Arc<Channel<u32>>,
}

impl Server {
    pub fn new(
        port: &str,
        request_channel: Arc<Channel<(Request, Arc<Channel<()>>)>>,
        release_channel: Arc<Channel<u32>>,
    ) -> Self {
        let addr = format!("0.0.0.0:{}", port);
        let listener =
            TcpListener::bind(&addr).expect("Não foi possível abrir o servidor na porta");

        Self {
            listener,
            request_channel,
            release_channel,
        }
    }

    pub fn start(&mut self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();

            let request_channel = Arc::clone(&self.request_channel);
            let release_channel = Arc::clone(&self.release_channel);

            thread::spawn(move || {
                // Espera o cliente enviar o request.
                let request = Server::parse_request(&mut stream, Operation::Request)
                    .expect("Primeira operação deve ser um REQUEST");

                // Cria um canal para cada request e envia para o coordenador para que ele
                // possa responder o request com o grant.
                let grant_channel = Arc::new(Channel::<()>::new());
                request_channel.send((request, Arc::clone(&grant_channel)));

                // Espera o coordenador responder o request com o grant.
                grant_channel.recv();
                // Envia o GRANT para o cliente.
                let grant = Request {
                    operation: Operation::Grant,
                    id: request.id,
                };
                stream.write_all(&grant.as_bytes()).unwrap();

                // Espera o cliente enviar o release.
                let release = Server::parse_request(&mut stream, Operation::Release)
                    .expect("Segunda operação deve ser um RELEASE");

                // Envia o release para o coordenador.
                release_channel.send(release.id);
            });
        }
    }

    pub fn parse_request(stream: &mut TcpStream, expected_operation: Operation) -> Option<Request> {
        let mut buffer = [0; 5];
        stream.read_exact(&mut buffer).unwrap();

        let request = Request::from(buffer);

        if request.operation != expected_operation {
            return None;
        } else {
            Some(request)
        }
    }
}
