#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(value) = number.iter().find(|v| **v >= from_base) {
        return Err(Error::InvalidDigit(*value));
    }

    let mut my_number: u32 = 0;
    for value in number {
        my_number = my_number * from_base + value;
    }

    let mut reminds: Vec<u32> = Vec::new();
    loop {
        reminds.push(my_number % to_base);
        my_number /= to_base;
        if my_number == 0 {
            break;
        }
    }
    reminds.reverse();

    Ok(reminds)
}
