use crate::dealer::Dealer;
use crate::member::Member;
use crate::player::Player;
use crate::Deck;

#[derive(Debug)]
pub struct PlayerList {
    players: Vec<Member>,
    current_player: usize,
}

impl PlayerList {
    pub(crate) fn new() -> PlayerList {
        let mut members: Vec<Member> = vec![];

        for i in 0..4 {
            members.push(Member::Player(Player::new(
                String::from("Player ") + &*i.to_string(),
            )))
        }
        members.push(Member::Dealer(Dealer::new("Jack Black the Dealer")));

        return PlayerList {
            players: members,
            current_player: 0,
        };
    }

    fn next_player(&self) -> usize {
        //TODO wrap counting to account for dealer and size of PlayerList (self)
        let mut index = self.current_player;
        index += 1; // increment
        index %= 6; // modulus
        println!("Previous index: {} Current index: {}", index - 1, index);
        return index;
    }
    pub(crate) fn turn(&mut self, ctx: &mut Deck) {
        self.players[self.current_player].turn(ctx);
        self.next_player();
    }
}
