use crate::dealer::Dealer;
use crate::member::Member;
use crate::player::Player;
use crate::Deck;
use std::ops::Add;
use std::{thread, time};

#[derive(Debug)]
/// Playerlist contains the current list of players and the current player indicated by current_index
pub struct PlayerList {
    pub players: Vec<Member>,
    pub current_index: usize,
}

impl PlayerList {
    pub fn new() -> PlayerList {
        let mut members: Vec<Member> = vec![];

        for i in 0..4 {
            members.push(Member::Player(Player::new(
                String::from("Player ").add(&*i.to_string()),
            )))
        }
        // Push a new dealer with a specific name to the end of player list
        members.push(Member::Dealer(Dealer::new("Jack Black the Dealer")));

        return PlayerList {
            players: members,
            current_index: 0,
        };
    }

    /// Removes all players from self.players and resets the index
    pub fn empty(&mut self) {
        self.players.drain(0..self.players.len());
        self.current_index = 0
    }

    /// Turn behaviour
    pub fn turn(&mut self, ctx: &mut Deck) {
        println!("{}", self.current_member());
        self.current_index += self.players[self.current_index].turn(ctx);
        println!("{:-<1$}", "", 50);
        thread::sleep(time::Duration::from_millis(2000));
    }

    /// returns the member of the current index
    pub fn current_member(&self) -> &Member {
        return &self.players[self.current_index];
    }
}
