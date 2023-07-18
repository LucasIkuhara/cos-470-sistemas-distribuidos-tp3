mod channel;
mod coordinator;
mod server;

use channel::Channel;
use clap::Parser;
use std::error::Error;
use std::sync::Arc;
use std::thread;

use coordinator::Coordinator;
use request::Request;
use server::Server;

#[derive(Parser)]
struct CoordinatorOptions {
    #[clap(short, long, default_value = "8080")]
    port: String,

    #[clap(short, long, default_value = "coordinator.log")]
    log: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: CoordinatorOptions = CoordinatorOptions::parse();

    // Arc é uma referência de contagem atômica que possibilita compartilhar variáveis
    // entre threads.
    let request_channel = Arc::new(Channel::<(Request, Arc<Channel<()>>)>::new());
    let release_channel = Arc::new(Channel::<u32>::new());

    let mut server = Server::new(
        &opts.port,
        Arc::clone(&request_channel),
        Arc::clone(&release_channel),
    );

    let coordinator = Arc::new(Coordinator::new(
        &opts.log,
        Arc::clone(&request_channel),
        Arc::clone(&release_channel),
    ));

    // Inicia uma thread para o terminal
    {
        let coordinator = Arc::clone(&coordinator);
        thread::spawn(move || coordinator.start_cli());
    }

    // Inicia uma thread para o consumidor
    {
        let coordinator = Arc::clone(&coordinator);
        thread::spawn(move || coordinator.start_consumer());
    }

    // Inicia o servidor TCP na thread principal
    server.start();

    Ok(())
}
