use crate::structs::weapon::{Spell, Sword, Dagger, Club, Bow, Axe};

// Purpose: Define the character traits and structs for the game
pub trait Character {
    fn attack(&self, target: &mut dyn Character);
    fn take_damage(&mut self, damage: i32);
    fn get_name(&self) -> String;
    fn get_weapon(&self) -> String;
    fn get_health(&self) -> i32;
}

pub struct Druid {
    pub name: String,
    pub weapon: Spell,
    pub health : i32,
}
// impl self
impl Druid {
    pub fn new() -> Druid {
        Druid {
            name: "Druid".to_string(),
            health: 51,
            weapon: Spell {
                name: "Thorn Whip".to_string(),
                damage: 6,
            },
        }
    }
}
// impl for the trait
impl Character for Druid {
    fn attack(&self, target: &mut dyn Character) {
        println!("{} attacks {} with a {}", self.name, target.get_name(), self.weapon.name);
        if target.get_health() - self.weapon.damage < 0 {
            target.take_damage(self.weapon.damage);
            println!("{} has no health left", target.get_name());
        } else {
            target.take_damage(self.weapon.damage);
            println!("{} has {} health left", target.get_name(), target.get_health());
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    fn get_name(&self) -> String{
        self.name.clone()
    }

    fn get_weapon(&self) -> String{
        self.weapon.name.clone()
    }

    fn get_health(&self) -> i32 {
        self.health
    }
}

pub struct Fighter {
    pub name: String,
    pub weapon: Sword,
    pub health : i32,
}

impl Fighter {
    pub fn new() -> Fighter {
        Fighter {
            name: "Fighter".to_string(),
            health: 62,
            weapon: Sword {
                name: "Long Sword".to_string(),
                damage: 8,
            },
        }
    }
}

impl Character for Fighter {
    fn attack(&self, target: &mut dyn Character) {
        println!("{} attacks {} with a {}", self.name, target.get_name(), self.weapon.name);
        if target.get_health() - self.weapon.damage < 0 {
            target.take_damage(self.weapon.damage);
            println!("{} has no health left", target.get_name());
        } else {
            target.take_damage(self.weapon.damage);
            println!("{} has {} health left", target.get_name(), target.get_health());
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_weapon(&self) -> String{
        self.weapon.name.clone()
    }

    fn get_health(&self) -> i32 {
        self.health
    }
}

pub struct Assassin {
    pub name: String,
    pub weapon: Dagger,
    pub health : i32,
}

impl Assassin {
    pub fn new() -> Assassin {
        Assassin {
            name: "Assassin".to_string(),
            health: 52,
            weapon: Dagger {
                name: "Dagger".to_string(),
                damage: 5,
            },
        }
    }
}
impl Character for Assassin {
    fn attack(&self, target: &mut dyn Character) {
        println!("{} attacks {} with a {}", self.name, target.get_name(), self.weapon.name);
        if target.get_health() - self.weapon.damage < 0 {
            target.take_damage(self.weapon.damage);
            println!("{} has no health left", target.get_name());
        } else {
            target.take_damage(self.weapon.damage);
            println!("{} has {} health left", target.get_name(), target.get_health());
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_weapon(&self) -> String {
        self.weapon.name.clone()
    }

    fn get_health(&self) -> i32 {
        self.health
    }
}

pub struct Orc {
    pub name: String,
    pub weapon: Axe,
    pub health : i32,
}

impl Orc {
    pub fn new() -> Orc {
        Orc {
            name: "Orc".to_string(),
            health: 54,
            weapon: Axe {
                name: "Hand Axe".to_string(),
                damage: 8,
            },
        }
    }
}
impl Character for Orc {
    fn attack(&self, target: &mut dyn Character) {
        println!("{} attacks {} with a {}", self.name, target.get_name(), self.weapon.name);
        if target.get_health() - self.weapon.damage < 0 {
            target.take_damage(self.weapon.damage);
            println!("{} has no health left", target.get_name());
        } else {
            target.take_damage(self.weapon.damage);
            println!("{} has {} health left", target.get_name(), target.get_health());
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_weapon(&self) -> String{
        self.weapon.name.clone()
    }

    fn get_health(&self) -> i32 {
        self.health
    }
}

pub struct Drow {
    pub name: String,
    pub weapon: Bow,
    pub health : i32,
}

impl Drow {
    pub fn new() -> Drow {
        Drow {
            name: "Drow".to_string(),
            health: 44,
            weapon: Bow {
                name: "Long Bow".to_string(),
                damage: 8,
            },
        }
    }
}
impl Character for Drow {
    fn attack(&self, target: &mut dyn Character) {
        println!("{} attacks {} with a {}", self.name, target.get_name(), self.weapon.name);
        if target.get_health() - self.weapon.damage < 0 {
            target.take_damage(self.weapon.damage);
            println!("{} has no health left", target.get_name());
        } else {
            target.take_damage(self.weapon.damage);
            println!("{} has {} health left", target.get_name(), target.get_health());
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_weapon(&self) -> String {
        self.weapon.name.clone()
    }

    fn get_health(&self) -> i32 {
        self.health
    }
}

pub struct Goblin {
    pub name: String,
    pub weapon: Club,
    pub health : i32,
}

impl Goblin {
    pub fn new() -> Goblin {
        Goblin {
            name: "Goblin".to_string(),
            health: 34,
            weapon: Club {
                name: "Club".to_string(),
                damage: 4,
            },
        }
    }
}
impl Character for Goblin {
    fn attack(&self, target: &mut dyn Character) {
        println!("{} attacks {} with a {}", self.name, target.get_name(), self.weapon.name);
        if target.get_health() - self.weapon.damage < 0 {
            target.take_damage(self.weapon.damage);
            println!("{} has no health left", target.get_name());
        } else {
            target.take_damage(self.weapon.damage);
            println!("{} has {} health left", target.get_name(), target.get_health());
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_weapon(&self) -> String {
        self.weapon.name.clone()
    }

    fn get_health(&self) -> i32 {
        self.health
    }
}