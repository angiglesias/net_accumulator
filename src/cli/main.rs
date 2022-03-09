use tcp_accul::*;

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
        // parse number
        let raw_number = line.unwrap();
        let number: i32 = match raw_number.trim().parse() {
            Ok(num) => {
                // stop condition;
                if num == 0 {
                    println!(">> Received number 0, which is the stop condition. Exiting...");
                    stream.shutdown(Shutdown::Both).unwrap();
                    break;
                }
                num
            }
            Err(_) => {
                println!(
                    ">> {} is not a valid integer, please try again with a new number",
                    raw_number.trim(),
                );
                continue;
            }
        };
        // send number
        match stream.write_all(&number.to_le_bytes()) {
            Ok(_) => {
                // flush stream
                match stream.flush() {
                    Ok(_) => (),
                    Err(err) => {
                        error!(
                            "Error sending number {} to server {}: {}",
                            number, address, err
                        );
                        stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    }
                }
            }
            Err(err) => {
                error!(
                    "Error sending number {} to server {}: {}",
                    number, address, err
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
