/* Problem 14: Longest Collatz sequence
 *
 * The following iterative sequence is defined for the set of positive integers:
 *
 * n → n/2 (n is even)
 * n → 3n + 1 (n is odd)
 *
 * Using the rule above and starting with 13, we generate the following sequence:
 * 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
 *
 * It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although
 * it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at
 * 1.
 *
 * Which starting number, under one million, produces the longest chain?
 *
 * NOTE: Once the chain starts the terms are allowed to go above one million. */

#![feature(step_by)]

extern crate num;
use num::Integer;
use std::thread;
use std::sync::mpsc::{Receiver, Sender, channel};

const MAX: u64 = 1_000_000;

fn main() {
    let master_rx = spawn_workers(MAX);

    let mut result = 0;
    let mut max    = 0;

    for WorkResult { number: num, result: current } in master_rx.iter() {
        if current > max {
            max = current;
            result = num;
        }
    }

    println!("{}", result);
}

fn spawn_workers(max: u64) -> Receiver<WorkResult> {
    use std::env::{self, VarError};

    let (master_tx, master_rx) = channel();

    let task_count: u64 = match env::var("NPROC") {
        Ok(num) => num.parse().unwrap(),
        Err(VarError::NotPresent) => 4,
        Err(err) => panic!("{}", err),
    };

    let per_task = max / task_count;

    for start in (1..max + 1).step_by(per_task) {
        let master_tx_clone = master_tx.clone();

        thread::spawn(move || {
            let end = start + per_task;
            collatz_worker((start, end), master_tx_clone);
        });
    }

    return master_rx;
}

struct WorkResult {
    number: u64,
    result: u32,
}

fn collatz_worker(numbers: (u64, u64), tx: Sender<WorkResult>) {
    let (start, end) = numbers;

    let mut current = 0;
    let mut max = 0;

    for num in start..end {
        let len = collatz_length(num);

        if len > max {
            current = num;
            max = len;
        }
    }

    tx.send(WorkResult { number: current, result: max as u32 }).unwrap();
}

fn collatz_length(number: u64) -> usize {
    let mut length = 1;
    let mut current_number = number;

    while current_number > 1 {
        current_number =
            if current_number.is_even() {
                current_number / 2
            } else {
                3 * current_number + 1
            };

        length += 1;
    }

    length
}
