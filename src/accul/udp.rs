use crate::accul::BaseOps;

use log::{debug, error, info};
use std::net::{SocketAddr, UdpSocket};

pub struct Accumulator {
    value: i32,
}

pub trait UdpOps {
    // Handle connection
    fn handle_udp(&mut self, socket: &mut UdpSocket, data: &[u8], peer_addr: SocketAddr);
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

impl UdpOps for Accumulator {
    fn handle_udp(&mut self, socket: &mut UdpSocket, data: &[u8], peer_addr: SocketAddr) {
        // read numbers using 4-byte each time
        let nums: Vec<i32> = (0..data.len()/std::mem::size_of::<i32>())
            .map(|x| {
                i32::from_le_bytes(
                    data[x * std::mem::size_of::<i32>()..(x + 1) * std::mem::size_of::<i32>()]
                        .try_into()
                        .unwrap(),
                )
            })
            .collect();
        debug!(
            "Received from client {} the following numbers to sum {:?}",
            peer_addr, nums
        );
        // update accumulator with aggregated value
        let sum = nums.iter().sum();
        info!("Adding {} to accumulator of client {}", sum, peer_addr);
        let current_value = self.sum(sum);
        info!(
            "Current accumulator value for client {} is: {}",
            peer_addr, current_value
        );
        // send current accumulator value to client
        match socket.send_to(&current_value.to_le_bytes(), peer_addr) {
            Ok(sent_bytes) => {
                if sent_bytes == std::mem::size_of::<i32>() {
                    info!(
                        "Sent accumulator value {} to client {}",
                        current_value, peer_addr
                    );
                } else {
                    error!(
                        "Response datagram not sent fully ({}/{} bytes sent",
                        sent_bytes,
                        std::mem::size_of::<i32>()
                    )
                }
            }
            Err(err) => {
                error!("Error sending response to client {}: {}", peer_addr, err);
            }
        }
    }
}
