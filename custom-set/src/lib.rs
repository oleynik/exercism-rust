use std::ptr::eq;

#[derive(Debug, Eq)]
pub struct CustomSet<T: Eq + Clone> {
    storage: Vec<T>,
}

impl<T: Eq + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        Self {
            storage: _input.to_vec(),
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.storage.iter().any(|e| e.eq(_element))
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.storage.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.is_empty() {
            true
        } else {
            self.storage.iter().map(|e| _other.contains(e)).all(|r| r)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        _other.storage.iter().map(|e| self.contains(e)).all(|r| !r)
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let vec: Vec<T> = _other
            .storage
            .iter()
            .filter(|&e| self.contains(e))
            .map(|e| e.clone())
            .collect();
        Self { storage: vec }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let result: Vec<T> = self
            .storage
            .iter()
            .filter(|&e| !_other.contains(e))
            .map(|e| e.clone())
            .collect();
        Self { storage: result }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut result = Self { storage: vec![] };
        self.storage
            .iter()
            .chain(_other.storage.iter())
            .for_each(|e| result.add(e.clone()));
        result
    }
}

impl<T: Eq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.storage.len() != other.storage.len() {
            return false;
        }
        other.storage.iter().map(|e| self.contains(e)).all(|r| r)
    }

    fn ne(&self, other: &Self) -> bool {
        !eq(self, other)
    }
}
