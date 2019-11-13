#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().cloned().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_3 = self.scores.to_vec();
        top_3.sort_by(|a, b| a.cmp(b).reverse());
        top_3.iter().cloned().take(3).collect()
    }
}
