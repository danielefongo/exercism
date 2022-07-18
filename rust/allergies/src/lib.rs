use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

pub struct Allergies {
    score: u32,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, IntEnum, Sequence)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().into_iter().any(|a| a == *allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        all::<Allergen>()
            .filter(|it| self.score & it.int_value() != 0)
            .collect()
    }
}
