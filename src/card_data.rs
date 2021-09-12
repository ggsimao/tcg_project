use super::entities::{DamageType, MagicSchool, MonsterData};
use DamageType::*;
use MagicSchool::*;

fn initialize_card_data() {
    let wisp = MonsterData::new(String::from("Wisp"), 1, 1, 1, Magical(Flame));

    let orc_grunt = MonsterData::new(String::from("Orc Grunt"), 3, 3, 3, Physical(0));
}
