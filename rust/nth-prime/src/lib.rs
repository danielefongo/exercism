struct PrimeNumbers {
    actual: u32,
}

impl PrimeNumbers {
    pub fn new() -> PrimeNumbers {
        PrimeNumbers { actual: 1 }
    }

    fn is_prime(&self) -> bool {
        if self.actual <= 3 {
            return self.actual > 1;
        }
        if self.actual % 2 == 0 || self.actual % 3 == 0 {
            return false;
        }
        let end = ((self.actual as f32).powf(0.5)) as u32;
        (5..=end)
            .step_by(6)
            .all(|it| self.actual % it != 0 && self.actual % (it + 2) != 0)
    }
}

impl Iterator for PrimeNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.actual += 1;
        while !self.is_prime() {
            self.actual += 1;
        }
        Some(self.actual)
    }
}

pub fn nth(n: u32) -> u32 {
    PrimeNumbers::new().nth(n as usize).unwrap()
}
