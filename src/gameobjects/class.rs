#[derive(Debug)]
pub enum Class {
    Warrior,
    Mage,
    Rogue,
}

impl Class {
    pub fn base_health(&self) -> i32 {
        match self {
            Class::Warrior => 100,
            Class::Mage => 80,
            Class::Rogue => 90,
        }
    }

    pub fn base_strength(&self) -> i32 {
        match self {
            Class::Warrior => 10,
            Class::Mage => 5,
            Class::Rogue => 7,
        }
    }
}
