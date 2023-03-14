use crate::Allergen::{Cats, Chocolate, Eggs, Peanuts, Pollen, Shellfish, Strawberries, Tomatoes};

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

impl Allergen {
    fn values() -> Vec<Allergen> {
        vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let a = *allergen as u32;
        a & self.score == a
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::values()
            .iter()
            .filter(|&a| self.is_allergic_to(a))
            .map(Allergen::clone)
            .collect()
    }
}
