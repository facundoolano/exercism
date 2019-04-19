use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut results = HashSet::new();

    for a in 1..sum {
        let powa = a.pow(2);
        for b in (a + 1)..(sum - a) {
            let c = sum - a - b;
            if powa + b.pow(2) == c.pow(2) {
                results.insert([a, b, c]);
            }
        }
    }

    results
}
