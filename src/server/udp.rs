use log::*;
use simple_logger::SimpleLogger;
use std::collections::hash_map::HashMap;
use std::env;
use std::net::{SocketAddr, UdpSocket};
use std::process::exit;
// use std::thread;

use net_accul::{
    accul::udp::{Accumulator, UdpOps},
    get_program_name,
};

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
    let mut listener = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();

    // map with accumulator instance for clients
    let mut accumulators: HashMap<SocketAddr, Accumulator> = HashMap::new();
    let mut buffer = [0u8; 2048];

    loop {
        let (bytes_recv, peer_addr) = match listener.recv_from(&mut buffer) {
            Ok(res) => res,
            Err(err) => {
                error!("Error receiving data from client: {}", err);
                continue;
            }
        };
        // Process incoming client datagram
        info!(
            "Processing incoming datagram from client {} on address {}",
            peer_addr,
            listener.local_addr().unwrap()
        );
        debug!("Received {} bytes from client {}", bytes_recv, peer_addr);
        // recover accumulator instance
        let client_accul: &mut Accumulator;
        if !accumulators.contains_key(&peer_addr) {
            accumulators.insert(peer_addr, Accumulator::default());
        }
        client_accul = accumulators.get_mut(&peer_addr).unwrap();
        // Handle message
        client_accul.handle_udp(&mut listener, &buffer[0..bytes_recv], peer_addr);

        // TODO test using threads => probable running condition on client messages
        // thread::spawn(move || {
        //     let mut accumulator = Accumulator::default();
        //     accumulator.handle_tcp(&mut stream);
        // });
    }
}
