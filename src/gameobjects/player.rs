use std::{
    io::{self, stdin, Write},
    thread::sleep,
};

use crate::gameobjects::main_constants::SLEEP_TIME;

use super::{class::Class, collectables::Collectable, enemy::Enemy, enemy::EnemyTrait};

pub struct Player {
    name: String,
    level: i32,
    class: Class,
    speed: i32,
    exp: i32,
    health: i32,
    strength: i32,
    defence: i32,
    gold: i32,
    inventory: Vec<Box<dyn Collectable>>,
}

impl Player {
    pub fn new(name: String, class: Class) -> Self {
        let health = class.base_health();
        let strength = class.base_strength();
        Player {
            name,
            level: 1,
            class,
            exp: 0,
            health,
            speed: 4,
            strength,
            defence: 0,
            gold: 0,
            inventory: Vec::new(),
        }
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
    pub fn attack(&mut self, enemy: &mut Enemy) {
        loop {
            println!("Please select any option");
            println!("1. Attack");
            println!("2. Use potion");
            println!("3. Run");
            print!("Your choice: ");
            io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt is printed before reading input
            let mut input: String = String::new();
            if stdin().read_line(&mut input).is_err() {
                println!("Failed to read line. Please try again.");
                continue;
            }

            let input: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a valid number!");
                    continue; // Re-prompt the user for valid input
                }
            };

            match input {
                1 => {
                    println!(
                        "You hit {} for {} damage!",
                        enemy.enemy_name(),
                        self.strength
                    );
                    sleep(SLEEP_TIME);
                    enemy.take_damage(self.strength);

                    if enemy.health() == 0 {
                        break;
                    }
                    enemy.attack(self);
                }
                2 => {
                    println!("You attacked {} with a potion!", enemy.enemy_name());
                    // add mechanics to check inventory for potions
                    break;
                }
                _ => {
                    println!("Invalid option!");
                    continue;
                }
            }
        }
    }
    pub fn gold(&self) -> i32 {
        self.gold
    }
    pub fn add_gold(&mut self, gold: i32) {
        self.gold += gold;
    }
    pub fn return_speed(&self) -> i32 {
        self.speed
    }

    pub fn add_item(&mut self, item: Box<dyn Collectable>) {
        self.inventory.push(item);
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("Class: {:?}", self.class);
        println!("Exp: {}", self.exp);
        println!("Speed: {}", self.speed);
        println!("Health: {}", self.health);
        println!("Strength: {}", self.strength);
        println!("Defence: {}", self.defence);
        println!("Gold: {}", self.gold);
    }

    pub fn level_up(&mut self) {
        self.level += 1;
    }
}
