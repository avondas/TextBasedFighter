mod structs;

use rand::seq::SliceRandom;
use std::io;
use structs::character::{Character, Druid, Fighter, Assassin, Orc, Drow, Goblin};

fn main() {
    let fighter = choose_hero();
    let enemy = choose_enemy();
    fight(fighter, enemy);
}

fn choose_hero() -> Box<dyn Character> {
    // print the options for the user to choose from
    println!("Choose your fighter: \n1. A Druid\n2. A Fighter\n3. An Assassin");

    // get the user input and store it in a variable
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    // the hero and give it a default value
    let mut fighter: Box<dyn Character> = Box::new(Druid::new());

    // check the input
    if input.trim() == "1" {
        fighter = Box::new(Druid::new());
    } else if input.trim() == "2" {
        fighter = Box::new(Fighter::new());
    } else if input.trim() == "3" {
        fighter = Box::new(Assassin::new());
    } else {
        input = "Invalid".to_string();
    }

    // is the input invalid, call the function again otherwise return the hero
    if input == "Invalid" {
        println!("Invalid choice");
        return choose_hero();
    } else {
        return fighter;
    }
}

fn choose_enemy() -> Box<dyn Character> {
    // print the options for the user to choose from
    println!("Pick a number, 1 2 or 3");

    // get the user input and store it in a variable
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    // the enemy and give it a default value
    let mut enemy: Box<dyn Character> = Box::new(Druid::new());

    // check the input
    if input.trim() == "1" {
        enemy = Box::new(Drow::new());
    } else if input.trim() == "2" {
        enemy = Box::new(Orc::new());
    } else if input.trim() == "3" {
        enemy = Box::new(Goblin::new());
    } else {
        input = "Invalid".to_string();
    }

    // is the input invalid, call the function again otherwise return the enemy
    if input == "Invalid" {
        println!("Invalid choice");
        return choose_enemy();
    } else {
        return enemy;
    }
}

fn fight(mut fighter: Box<dyn Character>, mut enemy: Box<dyn Character>) {

    print!("You are fighting a {}!\n", enemy.get_name());

    while fighter.get_health() > 0 && enemy.get_health() > 0 {
        fighter.attack(&mut *enemy);
        enemy.attack(&mut *fighter);
    }

    if fighter.get_health() > 0 {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}