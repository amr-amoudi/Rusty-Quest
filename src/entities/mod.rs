pub mod all_entities {
    pub mod player {
        pub struct Player {
            pub health: i16,
            pub name: String,
            pub gold: u16,
            pub damage_multiplayer: f32,
        }

        pub fn create_player(health: i16, name: String, gold: u16, damage_multiplayer: f32) -> Player {
           Player {
               health,
               name,
               gold,
               damage_multiplayer,
           }
        }
        
    }

    pub mod monsters {
        pub struct Monster {
            pub health: i16,
            pub name: String,
            pub max_gold_drop: u16,
            pub max_damage: i16,
        }
        
        pub fn create_monster(health: i16, name: String, max_gold_drop: u16, max_damage: i16) -> Monster{
            Monster {
                health,
                name,
                max_gold_drop,
                max_damage,
            }
        }
    }
}