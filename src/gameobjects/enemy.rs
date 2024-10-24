use super::player::Player;

#[derive(Debug)]
pub enum EnemyType {
    Rat,
    Orc,
    Skeleton,
}

// Define a trait for common behavior of all enemies
pub trait EnemyTrait {
    fn attack(&self, player: &mut Player); // Function to handle attack
    fn health(&self) -> i32; // Function to get health
    fn take_damage(&mut self, damage: i32); // Function to handle damage
}

// Struct to represent an enemy
#[derive(Debug)]
pub struct Enemy {
    enemy_type: EnemyType,
    health: i32,
    attack_power: i32,
    speed: i32,
}

// Implement the constructor method as an associated function
impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Rat => Self {
                enemy_type: EnemyType::Rat,
                health: 10,
                attack_power: 5,
                speed: 3,
            },
            EnemyType::Orc => Self {
                enemy_type: EnemyType::Orc,
                health: 20,
                speed: 2,
                attack_power: 15,
            },
            EnemyType::Skeleton => Self {
                enemy_type: EnemyType::Skeleton,
                health: 15,
                attack_power: 10,
                speed: 1,
            },
        }
    }

    pub fn return_speed(&self) -> i32 {
        self.speed
    }

    pub fn enemy_name(&self) -> String {
        format!("{:?}", self.enemy_type)
    }

    pub fn stats(&self) -> String {
        format!(
            "Enemy type: {:?}\nHealth: {}\nAttack power: {}",
            self.enemy_type, self.health, self.attack_power
        )
    }
}

// Implement EnemyTrait for Enemy
impl EnemyTrait for Enemy {
    fn attack(&self, player: &mut Player) {
        println!("{} attacks you!", self.enemy_name());
        player.take_damage(self.attack_power); // Reduce player health by self.attack_power;
    }

    fn health(&self) -> i32 {
        self.health
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health < 0 {
            self.health = 0; // Ensure health doesn't go below 0
            println!("You have slain {}!", self.enemy_name());
            // Drops and loot logic can be added here
        }
    }
}
