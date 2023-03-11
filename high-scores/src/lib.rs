#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores{scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(last) => Some(*last),
            None => None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(max) => Some(*max),
            None => None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut indexes: Vec<usize> = vec![];
        for _ in 0..3.min(self.scores.len()) {
            let mut max = 0;
            let mut max_index = 0;
            for i in 0..self.scores.len() {
                if self.scores[i] >= max && !indexes.contains(&i) {
                    max = self.scores[i];
                    max_index = i;
                }
            }
            indexes.push(max_index);
        }
        indexes.iter().map(|&i|self.scores[i]).collect()
    }
}
