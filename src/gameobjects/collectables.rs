use rand::{thread_rng, Rng};

use super::player::Player;

// Collectables
pub trait Collectable {
    fn collect(&self, player: &mut Player) -> String;
}

pub struct Coin {
    amount: i32,
}

impl Coin {
    const MAX_AMOUNT: i32 = 100;
    const MIN_AMOUNT: i32 = 1;
}

impl Collectable for Coin {
    fn collect(&self, player: &mut Player) -> String {
        let mut rng = thread_rng();
        player.add_gold(rng.gen_range(Coin::MIN_AMOUNT..=Coin::MAX_AMOUNT));
        "You've collected a coin!".to_string()
    }
}
