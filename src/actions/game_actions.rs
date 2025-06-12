pub mod actions {
    pub mod fight {
        use crate::entities::all_entities::monsters::Monster;
        use crate::entities::all_entities::player::Player;
        use crate::utils::clear_terminal::clear;
        use crate::utils::input::input_string;

        pub fn start_fight(player: &mut Player, monster: &mut Monster) {

            loop {
                let player_deal_damage = rand::random_range(10.0..100.0) * player.damage_multiplayer;
                monster.health -= player_deal_damage.floor() as i16;

                if player.health <= 0 {
                    println!("you lost");
                    break;
                }

                if monster.health <= 0 {
                    let won_gold: u16 = rand::random_range(2..monster.max_gold_drop);
                    player.gold += won_gold;
                    clear();
                    println!("{}", player_deal_damage.floor());
                    println!("you won!");
                    println!("you gained {} gold!", won_gold);
                    println!("\nyour current health is {}", player.health);
                    println!("your current gold balance is {}", player.gold);
                    break;
                }
                clear();
                println!("{} is on: {} health", monster.name ,monster.health);

                let monster_deal_damage = rand::random_range(1..monster.max_damage);
                player.health -= monster_deal_damage;

                println!("you are on: {} health", player.health);



                let want_to_fight = input_string(String::from("do you want to fight? yes/no"));
                if want_to_fight == "no" {
                    player.gold -= 30;
                    player.health -= 30;
                    break;
                }
            }
        }
    }
}