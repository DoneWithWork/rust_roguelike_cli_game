use std::{io::stdin, thread::sleep};

use rand::{thread_rng, Rng};

use crate::gameobjects::{
    enemy::{Enemy, EnemyType},
    main_constants::SLEEP_TIME,
};

use super::player::Player;

pub struct Room {
    id: i32,
    name: String,
    description: String,

    num_of_enemies: i32,
    num_of_treasure: i32,
}

/*
- should have a short description
- some ascll art
- background music

*/
impl Room {
    pub fn new(room_num: i32) -> Self {
        let mut rng = thread_rng();
        let num_of_enemies = rng.gen_range(2..5 as i32);
        let num_of_treasure = rng.gen_range(0..3 as i32);
        Room {
            id: room_num,
            name: String::from(""),
            description: String::from(""),
            num_of_enemies,
            num_of_treasure,
        }
    }

    pub fn actions(&self, player: &mut Player) {
        //get input from cli

        loop {
            println!("Actions available in Room Number 1");
            println!("Attack enemies {} --> 1", self.num_of_enemies);
            println!("Forage the room {} --> 2", self.num_of_treasure);
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a valid number!");
                    continue; // Re-prompt the user for valid input
                }
            };
            match input {
                1 => {
                    if self.num_of_enemies == 0 {
                        println!("No enemies to attack");
                        break;
                    }
                    sleep(SLEEP_TIME);
                    self.battle(player);
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
    fn forage_room(&self) {}
    fn battle(&self, player: &mut Player) {
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        let choice = rng.gen_range(0..3 as i32);
        let mut enemy: Enemy;
        //based on choice
        enemy = match choice {
            0 => Enemy::new(EnemyType::Rat),
            1 => Enemy::new(EnemyType::Orc),
            2 => Enemy::new(EnemyType::Skeleton),
            _ => {
                println!("Invalid input");
                panic!("Invalid enemy selection, Should Panic");
            }
        };
        println!("You have encountered an enemy: {}", enemy.enemy_name());
        player.attack(&mut enemy);

        //game loop for battle
        //player also have actions like attacking or using items
        // 1. Attack, 2. Potions, 3. Run --> not all the time can run --> reduced chances with each run
    }
}
