pub struct Suspicion {
    pub from: usize,
    pub suspected: usize,
    pub description: String,
    pub for_votes: usize,
    pub against_votes: usize,
}

impl Suspicion {
    pub fn new(from: usize, suspected: usize, description: String) -> Self {
        Self {
            from,
            suspected,
            description,
            for_votes: 0,
            against_votes: 0,
        }
    }
}

pub struct Mail {
    pub from: usize,
    pub to: usize,
    pub contents: String,
}
