use crate::dealer::Dealer;
use crate::member::Member;
use crate::player::Player;
use crate::Deck;
use std::ops::Add;

#[derive(Debug)]
pub struct PlayerList {
    pub(crate) players: Vec<Member>,
    pub(crate) current_index: usize,
}

impl PlayerList {
    pub(crate) fn new() -> PlayerList {
        let mut members: Vec<Member> = vec![];

        for i in 0..4 {
            members.push(Member::Player(Player::new(
                String::from("Player ").add(&*i.to_string()),
            )))
        }
        members.push(Member::Dealer(Dealer::new("Jack Black the Dealer")));

        return PlayerList {
            players: members,
            current_index: 0,
        };
    }

    pub(crate) fn empty(&mut self) {
        self.players.drain(0..self.players.len());
        self.current_index = 0
    }

    ///
    pub(crate) fn turn(&mut self, ctx: &mut Deck) {
        println!("{}", self.current_member());
        self.current_index += self.players[self.current_index].turn(ctx);
    }

    ///
    pub(crate) fn current_member(&self) -> &Member {
        return &self.players[self.current_index];
    }
}
