mod entities;
mod utils;
mod actions;

use entities::all_entities::*;
use crate::utils::input::input_string;
use crate::utils::clear_terminal::clear;
use crate::actions::game_actions::all_game_actions::*;


fn main() {
    println!("🦀 Welcome to Rusty Quest");
    let player_name = input_string(String::from("Enter your name: "));
    let mut worm = monsters::create_monster(50, String::from("🪱 worm"), 60, 20);
    let mut my_player = player::create_player(100, String::from(player_name.trim()), 0, 1.0);

    let want_to_fight = input_string(String::from("do you want to fight? yes/no"));

    match want_to_fight.as_str() {
        "yes" => {
            clear();
            fight::start_fight(&mut my_player, &mut worm);
        },
        "no" => println!("no he doesn't want to"),
        _ => println!("unexpected error"),
    }

    let want_to_heal = input_string(String::from("do you want to heal? yes/no"));
    
    match want_to_heal.as_str() {
        "yes" => {
            clear();
            heal::start_heal(&mut my_player);
        },
        "no" => println!("no"),
        _ => println!("unexpected error"),
    }
    
    if my_player.health <= 0 {
        println!("you lost");
    }

}
