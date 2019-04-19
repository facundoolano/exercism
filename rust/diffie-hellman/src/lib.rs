use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    binary_modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    binary_modular_pow(b_pub, a, p)
}

// https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
fn binary_modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus
    }
    result
}
