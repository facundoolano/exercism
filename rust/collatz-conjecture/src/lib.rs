#![feature(iter_unfold)]
use std::iter::successors;

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    Some(successors(Some(n), step).count())
}

fn step(n: u64) -> Option<u64> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        Some(3 * n + 1)
    }
}
