pub mod tcp;
pub mod udp;

// Base Accumulator operation trait
pub trait BaseOps {
    // Accumulate sums number to accumulator value and returns updaterd value
    fn sum(&mut self, n: i32) -> i32;
    // Return current acumulator value
    fn get(&self) -> i32;
}