pub struct Score {
    l_score: usize,
    r_score: usize,
}

impl Score {
    pub fn new() -> Score {
        Score {
            l_score: 0,
            r_score: 0,
        }
    }
}
