pub struct ScoreKeeper {
    scores: Vec<u32>,
    max_scores: usize,
}

impl ScoreKeeper {
    pub fn new(max_scores: usize) -> Self {
        if max_scores == 0 {
            panic!("Cannot create a ScoreKeeper with zero capacity!");
        }
        ScoreKeeper {
            scores: Vec::with_capacity(max_scores),
            max_scores,
        }
    }

    pub fn add_score(&mut self, score: u32) {
        if self.scores.len() >= self.max_scores {
            panic!("Cannot add score: ScoreKeeper is full!");
        }
        self.scores.push(score);
    }

    pub fn get_scores(&self) -> &[u32] {
        &self.scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_scores_within_limit() {
        let mut keeper = ScoreKeeper::new(2);
        keeper.add_score(88);
        keeper.add_score(33);
        assert_eq!(keeper.get_scores(), &[88, 33]);
    }

    #[test]
    #[should_panic]
    fn adding_score_to_full_keeper_panics() {
        let mut keeper = ScoreKeeper::new(1);
        keeper.add_score(80);
        keeper.add_score(71); // Panic
    }

    #[test]
    #[should_panic(expected = "ScoreKeeper is full")]
    fn adding_to_full_keeper_panics_with_specific_message() {
        let mut keeper = ScoreKeeper::new(1);
        keeper.add_score(88);
        keeper.add_score(99); // Panic
    }

    #[test]
    #[should_panic(expected = "zero capacity")] // checks the constructor panic
    fn new_score_keeper_with_zero_capacity_panics() {
        ScoreKeeper::new(0); // Panic
    }
}
