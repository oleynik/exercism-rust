#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank || self.position.file == other.position.file {
            return true;
        }
        let self_diff = self.position.rank - self.position.file;
        let other_diff = other.position.rank - other.position.file;
        if self_diff == other_diff {
            return true;
        }
        let self_sum = self.position.rank + self.position.file;
        let other_sum = other.position.rank + other.position.file;
        self_sum == other_sum
    }
}
