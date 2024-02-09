mod structs;

use std::io;
use structs::character::{Character, Druid, Fighter, Assassin, Orc, Drow, Goblin};
use rand::seq::SliceRandom;
use rand::thread_rng;


fn main() {
    // have user choose the hero and the enemy
    let fighter = choose_hero();
    let enemy = choose_enemy();

    // pass the hero name along for checking the defeat condition
    let fighter_name = fighter.get_name();

    fight(fighter_name, fighter, enemy);
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

fn fight(fighter_name: String, mut fighter: Box<dyn Character>, mut enemy: Box<dyn Character>) {

    print!("You are fighting a(n) {}!\n\n", enemy.get_name());

    // decide who goes first
    let mut first_up = first_to_go(&mut fighter, &mut enemy);
    print!("{} goes first!\n\n", first_up.get_name());

    // instantiate the first character
    let mut first;

    // instantiate the second character
    let mut second;

    // make sure they go up in the right order
    if first_up.get_name() == fighter.get_name() {
        first = fighter;
        second = enemy;
    } else {
        first = enemy;
        second = fighter;
    }

    // fight until one of the characters has no health left
    while first.get_health() > 0 && second.get_health() > 0 {
        first.attack(&mut *second);
        if second.get_health() <= 0 {
            break;
        }
        second.attack(&mut *first);

        // wait for the user to press enter before continuing
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
    }

    // print the result of the fight
    if first.get_health() > 0 && first.get_name() == fighter_name {
        println!("You win!");
    } else if second.get_health() > 0 && second.get_name() == fighter_name {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}

fn first_to_go<'a>(mut fighter: &'a mut Box<dyn Character>, mut enemy: &'a mut Box<dyn Character>) -> &'a mut Box<dyn Character> {
    // decide who goes first by shuffling the order
    let mut rng = thread_rng();
    let mut order = vec![&fighter, &enemy];
    order.shuffle(&mut rng);

    // return the first character
    if order[0].get_name() == fighter.get_name() {
        return fighter;
    } else {
        return enemy;
    }
}