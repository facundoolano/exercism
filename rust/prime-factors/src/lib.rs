pub fn factors(n: u64) -> Vec<u64> {

    let mut result: Vec<u64> = Vec::new();
    let mut reminder = n;
    let mut divisor = 2;

    while reminder > 1 {
        while reminder % divisor == 0 {
            reminder = reminder / divisor;
            result.push(divisor);
        }
        divisor += 1;
    }

    result
}
