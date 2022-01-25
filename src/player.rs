use crate::Card;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
    score: u8,
}

impl Player {
    pub(crate) fn new(name: String) -> Player {
        Player {
            name,
            hand: vec![],
            score: 0,
        }
    }
    fn hit(&self) {
        todo!()
    }
    fn pass(&self) {
        todo!()
    }
}
