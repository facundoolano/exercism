pub fn nth(n: u32) -> u32 {
    let mut seen_primes = 0;
    let mut current = 2;
    while seen_primes < n {
        current += 1;
        if is_prime(current) {
            seen_primes += 1;
        }
    }

    current
}

fn is_prime(n: u32) -> bool {
    for div in 2..(n / 2 + 1) {
        if n % div == 0 {
            return false;
        }
    }
    true
}
