use crate::player::Player;

#[derive(Debug)]
pub struct PlayerList {
    players: Vec<Player>,
    current_player: u8,
}

impl PlayerList {
    pub(crate) fn new() -> PlayerList {
        let mut players: Vec<Player> = vec![];

        for i in 0..4 {
            players.push(Player::new(String::from("Player ") + &*i.to_string()))
        }

        return PlayerList {
            players,
            current_player: 0,
        };
    }

    fn next_player(&mut self) -> Option<Player> {
        //self.current_player += 1 // increment
        todo!()
    }
}
