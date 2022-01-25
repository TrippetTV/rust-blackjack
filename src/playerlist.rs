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

        let current_player = &players[0];

        return PlayerList {
            players,
            current_player: 0,
        };
    }

    fn next_player(&self) -> u8 {
        let mut index = self.current_player;
        index += 1; // increment
        index %= 6; // modulus
        println!("Current player index: {}", index);
        return index;
    }
    pub(crate) fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }
}
