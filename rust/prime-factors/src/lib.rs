pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut counter = 2;

    while n > 1 {
        if n % counter == 0 {
            factors.push(counter);
            n /= counter;
        } else {
            counter += 1;
        }
    }
    factors
}
