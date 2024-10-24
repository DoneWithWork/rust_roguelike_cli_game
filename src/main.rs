mod gameobjects;
use gameobjects::class::Class;

use gameobjects::player::Player;
use gameobjects::room::Room;

use std::io::{self, stdin, Write};

const NUM_OF_ROOMS: i32 = 5;
fn main() {
    // let enemy = Enemy::new(EnemyType::Orc);
    // println!("{}", enemy.stats());
    let mut player = character_selection();
    for room in 0..NUM_OF_ROOMS {
        let new_room = Room::new(room);
        new_room.actions(&mut player);
    }
    game_loop();
}

fn character_selection() -> Player {
    loop {
        let mut input = String::new();
        print!("Please select a character (1: Warrior, 2: Mage, 3: Rogue): ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt is printed before reading input

        // Read the input
        if stdin().read_line(&mut input).is_err() {
            println!("Failed to read line. Please try again.");
            continue; // Retry if reading input fails
        }

        let input = input.trim(); // Trim whitespace
        let player: Player;
        player = match input {
            "1" => {
                println!("You selected Warrior.");
                // Proceed with Warrior selection logic
                Player::new(String::from("Warrior"), Class::Warrior)
            }
            "2" => {
                println!("You selected Mage.");
                // Proceed with Mage selection logic
                Player::new(String::from("Mage"), Class::Mage)
            }
            "3" => {
                println!("You selected Rogue.");
                // Proceed with Rogue selection logic
                Player::new(String::from("Rogue"), Class::Rogue)
            }
            _ => {
                println!("Invalid selection. Please enter 1, 2, or 3.");
                continue;
                // Loop again for valid input
            }
        };
        return player;
    }
}

fn game_loop() {
    println!("Welcome To Dungeon");
    // for the game can spawn have many rooms, advance to main boss room no running
}

// different scenarios
// diff number of rounds
// diff actions can do in the room
// room 1, room 2, room 3, can go to multiple rooms
// each room can have things to do, like fight, or collect treasure,
// cannot exit room until enemies are defeated
// acttack enemies (4) --> will have num of enemies must defeat all to ensure  can go to next room
