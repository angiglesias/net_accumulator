use crate::accul::BaseOps;

use log::{debug, error, info};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub struct Accumulator {
    value: i32,
}

pub trait TcpOps {
    // Handle connection
    fn handle_tcp(&mut self, stream: &mut TcpStream);
}

impl BaseOps for Accumulator {
    fn sum(&mut self, n: i32) -> i32 {
        self.value += n;
        self.value
    }

    fn get(&self) -> i32 {
        self.value
    }
}

impl Default for Accumulator {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl TcpOps for Accumulator {
    fn handle_tcp(&mut self, stream: &mut TcpStream) {
        // read numbers using 4-byte each time
        let mut header = [0 as u8; 4];
        // do while, all code happens inside of match
        while match stream.read_exact(&mut header) {
            Ok(_) => {
                // Decode header containing number of nums to come
                let datalen = u32::from_le_bytes(header) as usize;
                // read numbers
                let coded_nums: &mut [u8] = &mut vec![0; datalen * std::mem::size_of::<i32>()];
                match stream.read_exact(coded_nums) {
                    Ok(_) => {
                        // decode numbers and sum
                        let nums: Vec<i32> = (0..(datalen))
                            .map(|x| {
                                i32::from_le_bytes(
                                    coded_nums[x * std::mem::size_of::<i32>()
                                        ..(x + 1) * std::mem::size_of::<i32>()]
                                        .try_into()
                                        .unwrap(),
                                )
                            })
                            .collect();
                        debug!(
                            "Received from client {} the following numbers to sum {:?}",
                            stream.peer_addr().unwrap(),
                            nums
                        );
                        // update accumulator with aggregated value
                        let sum = nums.iter().sum();
                        info!(
                            "Adding {} to accumulator of client {}",
                            sum,
                            stream.peer_addr().unwrap()
                        );
                        let current_value = self.sum(sum);
                        info!(
                            "Current accumulator value for client {} is: {}",
                            stream.peer_addr().unwrap(),
                            current_value
                        );
                        // send updated accumulator
                        match stream.write_all(&current_value.to_le_bytes()) {
                            Ok(_) => {
                                // ensure that stream data is sent
                                match stream.flush() {
                                    Ok(_) => true,
                                    Err(err) => {
                                        error!(
                                            "Error flushing updated accumulator to client {}: {}",
                                            stream.peer_addr().unwrap(),
                                            err
                                        );
                                        false
                                    }
                                }
                            }
                            Err(err) => {
                                error!(
                                    "Error writing updated accumulator to client {}: {}",
                                    stream.peer_addr().unwrap(),
                                    err
                                );
                                // stop loop iteration
                                stream.shutdown(Shutdown::Both).unwrap();
                                false
                            }
                        }
                    }
                    Err(err) => {
                        error!(
                            "Error reading {} numbers sent by client {}: {}",
                            datalen,
                            stream.peer_addr().unwrap(),
                            err
                        );
                        stream.shutdown(Shutdown::Both).unwrap();
                        false
                    }
                }
            }
            Err(_) => {
                // close connection and stop loop
                error!(
                    "Connection with client {} closed!",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                // stop loop iteration
                false
            }
        } {}
    }
}
