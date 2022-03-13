use bytes::{BufMut, BytesMut};
use net_accul::*;

use log::*;
use simple_logger::SimpleLogger;
use std::env;
use std::io::{self, BufRead, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::process::exit;
use std::time::Duration;

fn main() {
    // setup logging
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!(
            "Invalid number of arguments. {} takes 2 args, received {}",
            get_program_name("tcp1cli"),
            args.len() - 1
        );
        exit(1);
    }
    // parse remote socket address
    let address: SocketAddr = format!("{}:{}", args[1], args[2])
        .parse()
        .expect(format!("{}:{} is a invalid remote address", args[1], args[2]).as_str());

    // create socket connection
    let mut stream = match TcpStream::connect_timeout(&address, Duration::from_secs(10)) {
        Ok(stream) => stream,
        Err(err) => {
            error!("Error connecting to server {}: {}", address, err);
            exit(1);
        }
    };

    // read numbers from stdin and send over tcp
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut resp = [0 as u8; 4];

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
            stream.shutdown(Shutdown::Both).unwrap();
            break;
        }
        debug!("Sending numbers {:?} to server {}", numbers, address);
        // encode numbers
        let mut buffer = BytesMut::with_capacity(
            numbers.len() * std::mem::size_of::<i32>() + std::mem::size_of::<u32>(),
        );
        // put header with quantity of numbers to send
        buffer.put_slice(&(numbers.len() as u32).to_le_bytes());
        // encode numbers
        numbers
            .iter()
            .for_each(|num| buffer.put_slice(&num.to_le_bytes()));
        // send number
        match stream.write_all(&buffer[..]) {
            Ok(_) => {
                // flush stream
                match stream.flush() {
                    Ok(_) => (),
                    Err(err) => {
                        error!(
                            "Error sending numbers {:?} to server {}: {}",
                            &numbers[..],
                            address,
                            err
                        );
                        stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    }
                }
            }
            Err(err) => {
                error!(
                    "Error sending number {:?} to server {}: {}",
                    &numbers[..],
                    address,
                    err
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
        // parse response
        match stream.read_exact(&mut resp) {
            Ok(_) => {
                let accul = i32::from_le_bytes(resp);
                println!(">> Received {} from server", accul);
            }
            Err(err) => {
                error!("Error receiving data from server: {}", err);
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }

        // continue loop
        println!(">> Type a integer number to send to the server");
    }

    info!("Closed connection with server {}", address);
}
