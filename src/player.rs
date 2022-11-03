#[derive(Debug)]
pub struct Player {
    name: String,
    tokens: usize,
    kind: PlayerType
}

#[derive(Debug)]
pub enum PlayerType {
    PC,
    Person
}

impl Player {
    pub fn new(name: String, tokens: usize, kind: PlayerType) -> Self {
        Player {
            name,
            tokens,
            kind
        }
    }
}