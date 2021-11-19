use super::entities::{DamageType, MagicSchool, MonsterData};
use MagicSchool::*;
use specs::prelude::*;

pub fn initialize_card_data(ecs: &mut World) {
    let mut monsters: Vec<MonsterData> = Vec::<MonsterData>::new();

    let wisp = MonsterData::new(String::from("Wisp"), 1, 1, 1, DamageType::new(0, Flame));
    let orc_grunt = MonsterData::new(String::from("Orc Grunt"), 3, 3, 3, DamageType::new(0, Physical));

    monsters.push(wisp);
    monsters.push(orc_grunt);

    ecs.insert(monsters);
}