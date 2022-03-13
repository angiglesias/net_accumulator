use bytes::{BufMut, BytesMut};
use net_accul::*;

use log::*;
use simple_logger::SimpleLogger;
use std::env;
use std::io::{self, BufRead};
use std::net::{SocketAddr, UdpSocket};
use std::process::exit;
use std::time::Duration;

fn main() {
    // setup logging
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!(
            "Invalid number of arguments. {} takes 2 args, received {}",
            get_program_name("udpcli"),
            args.len() - 1
        );
        exit(1);
    }
    // parse remote socket address
    let address: SocketAddr = format!("{}:{}", args[1], args[2])
        .parse()
        .expect(format!("{}:{} is a invalid remote address", args[1], args[2]).as_str());

    // create socket connection
    let port: u16 = if args.len() > 3 {
        args[3].parse().unwrap_or_default()
    } else {
        0
    };
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", port)) {
        Ok(socket) => socket,
        Err(err) => {
            error!("Error creating udp socket: {}", err);
            exit(1);
        }
    };
    // "connect" socket to remote server
    match socket.connect(&address) {
        Ok(res) => res,
        Err(err) => {
            error!(
                "Error establishing communication with remote {}: {}",
                address, err
            );
            exit(1);
        }
    }
    // set non-block mode for response
    socket
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();

    // read numbers from stdin and send over tcp
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut resp = [0 as u8; std::mem::size_of::<i32>()];

    println!(">> Type a integer number to send to the server");
    while let Some(line) = lines.next() {
        // parse numbers
        let numbers: Vec<i32> = line
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter_map(|s| match s.parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => {
                    warn!("{} is not a valid number, will be ignored", s);
                    None
                }
            }) // filter and ignore invalid numbers
            .collect();
        // check if valid numbers where received
        if numbers.is_empty() {
            println!(">> No valid numbers detected. Please, try again");
            continue;
        }
        // check stop condition
        if numbers.first().unwrap().clone() == 0 {
            println!("First number typed is a 0. Exiting program...");
            break;
        }
        debug!("Sending numbers {:?} to server {}", numbers, address);
        // encode numbers
        let mut buffer = BytesMut::with_capacity(
            numbers.len() * std::mem::size_of::<i32>() + std::mem::size_of::<u32>(),
        );
        numbers
            .iter()
            .for_each(|num| buffer.put_slice(&num.to_le_bytes()));
        // send number
        match socket.send(&buffer[..]) {
            Ok(bytes_sent) => {
                // flush stream
                if bytes_sent != buffer.len() {
                    error!(
                        "Error sending numbers {:?} to server {}: Message was not sent completed (sent {}/{} bytes)",
                        &numbers[..],
                        address,
                        bytes_sent,
                        buffer.len()
                    );
                    break;
                }
                info!("Sent numbers successfully to server {}", address);
            }
            Err(err) => {
                error!(
                    "Error sending number {:?} to server {}: {}",
                    &numbers[..],
                    address,
                    err
                );
                break;
            }
        }
        // parse response
        match socket.recv(&mut resp) {
            Ok(bytes_recv) => {
                if bytes_recv != std::mem::size_of::<i32>() {
                    error!(
                        "Response from server is not full (received {}/{} bytes)",
                        bytes_recv,
                        std::mem::size_of::<i32>()
                    );
                    break;
                }
                let accul = i32::from_le_bytes(resp);
                println!(">> Received {} from server", accul);
            }
            Err(err) => {
                error!("Error receiving data from server: {}", err);
                break;
            }
        }

        // continue loop
        println!(">> Type a integer number to send to the server");
    }

    info!("Closed connection with server {}", address);
}
