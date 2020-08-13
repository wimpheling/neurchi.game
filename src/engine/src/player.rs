use crate::n::N;

pub struct Player {
    ns: Vec<N>,
}

impl Player {
    pub fn new() -> Self {
        Player { ns: Vec::new() }
    }
}
