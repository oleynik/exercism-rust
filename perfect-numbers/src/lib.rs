#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = (1..=num / 2).filter(|i| num % i == 0).sum::<u64>();
    if sum == num {
        return Some(Classification::Perfect);
    }
    if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}
