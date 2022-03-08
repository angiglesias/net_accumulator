use log::*;
use simple_logger::SimpleLogger;
use std::env;
use std::net::TcpListener;
use std::process::exit;
use std::thread;

use tcp_accul::accul::tcp::{Accumulator, TcpOps};
use tcp_accul::get_program_name;

fn main() {
    // setup logging
    SimpleLogger::new().init().unwrap();

    // process arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!(
            "Invalid number of arguments. {} takes 2 args, received {}",
            get_program_name("tcp1ser"),
            args.len() - 1
        );
        exit(1);
    }
    // parse port number
    let port: u16 = args[1].parse().expect("Port number is not an valid uint16");
    info!("Server listening to incoming connection on port {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // Process incoming client connection
                info!(
                    "Processing incoming connection from client {} on address {}",
                    stream.peer_addr().unwrap(),
                    stream.local_addr().unwrap()
                );
                thread::spawn(move || {
                    let mut accumulator = Accumulator::default();
                    accumulator.handle_tcp(&mut stream);
                });
            }
            Err(err) => {
                error!("Error processing incoming client connection: {}", err)
            }
        }
    }
}
