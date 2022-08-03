pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|it| {
            factors
                .into_iter()
                .filter(|&f| f != &u32::MIN)
                .any(|&f| it % f == 0)
        })
        .sum()
}
