use specs::prelude::*;
use specs_derive::*;
use std::any;

#[derive(Component)]
pub enum HeroClass {
    Mage,
    Warrior,
    Ranger,
    Rogue,
    Priest,
}

#[derive(Component)]
pub enum CardHolder {
    MonsterCard(Monster),
    MagicCard(Magic),
}

impl CardHolder {
    pub fn name(&self) -> String {
        match self {
            CardHolder::MonsterCard(c) => c.data().name(),
            CardHolder::MagicCard(c) => c.data().name(),
            _ => {
                panic!("Invalid card type!");
            }
        }
    }

    pub fn hidden(&self) -> bool {
        match self {
            CardHolder::MonsterCard(c) => c.hidden(),
            CardHolder::MagicCard(c) => c.hidden(),
            _ => {
                panic!("Invalid card type!");
            }
        }
    }
    pub fn reveal(&mut self) -> () {
        match self {
            CardHolder::MonsterCard(c) => c.reveal(),
            CardHolder::MagicCard(c) => c.reveal(),
            _ => {
                panic!("Invalid card type!");
            }
        }
    }
}

// impl Iterator for  CardHolder {
//     // we will be counting with usize
//     type Item = usize;

//     // next() is the only required method
//     fn next(&mut self) -> Option<Self::Item> {
//         // Increment our count. This is why we started at zero.
//         self.count += 1;

//         // Check to see if we've finished counting or not.
//         if self.count < 6 {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// pub impl  CardHolder {
//     pub fn play_card_on_field(&self, board: &mut Board, target: usize) {
//         match self {
//             _(x) =>
//         }
//     }
//     fn play_card_on_hero(&self, board: &mut Board) {}
//     fn data(&self) -> Box<&dyn CardData>;
//     fn id(&self) -> u32;
// }

pub enum Target {
    Friendly(TargetType),
    Enemy(TargetType),
}

pub enum TargetType {
    Hero,
    Monster(usize),
}

#[derive(Copy, Clone)]
pub struct DamageType {
    range: u32,
    school: MagicSchool,
}

impl DamageType {
    pub fn new(range: u32, school: MagicSchool) -> DamageType {
        DamageType { range, school }
    }

    pub fn name(&self) -> String {
        let typename: String = match self.school {
            MagicSchool::Physical => "PHYS.".to_string(),
            MagicSchool::Flame => "FLAME".to_string(),
            MagicSchool::Frost => "FROST".to_string(),
            MagicSchool::Lightning => "LIGHTN.".to_string(),
            MagicSchool::Shadow => "SHADOW".to_string(),
            MagicSchool::Light => "LIGHT".to_string(),
            _ => "".to_string(),
        };
        format!("{}R {}", self.range, typename)
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self.school {
            MagicSchool::Physical => (255, 255, 255),
            MagicSchool::Flame => (255, 140, 0),
            MagicSchool::Frost => (0, 128, 128),
            MagicSchool::Lightning => (0, 0, 139),
            MagicSchool::Shadow => (128, 0, 128),
            MagicSchool::Light => (250, 250, 210),
            _ => (0, 0, 0),
        }
    }
}

#[derive(Copy, Clone)]
pub enum MagicSchool {
    Physical,
    Flame,
    Frost,
    Lightning,
    Shadow,
    Light,
}

pub trait Card {
    fn play_card_on_field(&self, board: &mut Board, target: usize) {}
    fn play_card_on_hero(&self, board: &mut Board) {}
    fn data(&self) -> Box<&dyn CardData>;
    fn id(&self) -> u32;
    fn hidden(&self) -> bool;
    fn reveal(&mut self) -> ();
}

pub trait CardData {
    fn name(&self) -> String {
        String::new()
    }
    fn as_monster(&self) -> Option<&MonsterData> {
        None
    }
    fn as_magic(&self) -> Option<&MagicData> {
        None
    }
}

#[derive(Component)]
pub struct Effect {
    source: CardHolder,
    targets: Vec<CardHolder>,
    effect: fn(usize),
}

#[derive(Clone)]
pub struct MonsterData {
    name: String,
    base_cost: i32,
    base_health: i32,
    base_damage: i32,
    attack_type: DamageType,
}

impl MonsterData {
    pub fn new(
        name: String,
        cost: i32,
        health: i32,
        damage: i32,
        attack_type: DamageType,
    ) -> MonsterData {
        MonsterData {
            name: name,
            base_cost: cost,
            base_health: health,
            base_damage: damage,
            attack_type: attack_type,
        }
    }

    pub fn base_cost(&self) -> i32 {
        self.base_cost
    }

    pub fn base_health(&self) -> i32 {
        self.base_health
    }

    pub fn base_damage(&self) -> i32 {
        self.base_damage
    }

    pub fn attack_type(&self) -> DamageType {
        self.attack_type
    }
}

impl CardData for MonsterData {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn as_monster(&self) -> Option<&MonsterData> {
        Some(self)
    }
}

// impl any::Any for MonsterData {
//     fn type_id(&self) -> std::any::TypeId {
//         any::TypeId::of::<MonsterData>()
//     }
// }

#[derive(Clone, Component)]
pub struct Monster {
    id: u32,
    cost: i32,
    health: i32,
    damage: i32,
    data: MonsterData,
    hidden: bool,
}

impl Monster {
    pub fn new(id: u32, data: MonsterData) -> Monster {
        Monster {
            id: id,
            cost: data.base_cost(),
            health: data.base_health(),
            damage: data.base_damage(),
            data: data,
            hidden: true,
        }
    }

    pub fn cost(&self) -> i32 {
        self.cost
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }
}

impl Card for Monster {
    fn play_card_on_field(&self, board: &mut Board, target: usize) {
        board.field[target] = Some(self.clone());
    }

    fn data(&self) -> Box<&dyn CardData> {
        Box::new(&self.data)
    }

    fn play_card_on_hero(&self, board: &mut Board) {}

    fn id(&self) -> u32 {
        self.id
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn reveal(&mut self) -> () {
        self.hidden = false;
    }
}

#[derive(Component)]
pub struct Magic {
    id: u32,
    cost: i32,
    data: MagicData,
    hidden: bool,
}

impl Card for Magic {
    fn data(&self) -> Box<&dyn CardData> {
        Box::new(&self.data)
    }

    fn play_card_on_hero(&self, board: &mut Board) {}

    fn id(&self) -> u32 {
        self.id
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn reveal(&mut self) -> () {
        self.hidden = false;
    }
}

#[derive(Component)]
pub struct MagicData {
    name: String,
    base_cost: i32,
    effect: Box<Effect>,
}

impl CardData for MagicData {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn as_magic(&self) -> Option<&MagicData> {
        Some(self)
    }
}

#[derive(Component)]
pub struct Hero {
    id: u8,
    base_health: i32,
    health: i32,
    class: HeroClass,
}

impl Hero {
    pub fn new(id: u8, base_health: i32, health: i32, class: HeroClass) -> Hero {
        Hero {
            id: id,
            base_health: base_health,
            health: health,
            class: class,
        }
    }

    fn id(&self) -> u8 {
        self.id
    }
}

#[derive(Component)]
pub struct Board {
    id: u8,
    hero: Hero,
    field: [Option<Monster>; 5],
    hand: Vec<CardHolder>,
    deck: Vec<CardHolder>,
    graveyard: Vec<CardHolder>,
    highlighted: (u8, i32),
}

impl Board {
    pub fn new(id: u8, hero: Hero, deck: Vec<CardHolder>) -> Board {
        let mut ret = Board {
            id: id,
            hero: hero,
            field: [None, None, None, None, None],
            hand: vec![],
            deck: deck,
            graveyard: vec![],
            highlighted: (3, 0),
        };

        ret
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn field(&self) -> [Option<Monster>; 5] {
        self.field.clone()
    }

    pub fn hand(&self) -> &Vec<CardHolder> {
        &self.hand
    }

    pub fn highlighted(&self) -> (u8, i32) {
        self.highlighted
    }

    pub fn change_highlighted(&mut self, new_highlighted: (u8, i32)) {
        self.highlighted = new_highlighted;
    }

    pub fn draw_card(&mut self) {
        match self.deck.pop() {
            Some(mut x) => {
                if self.id == 0 {
                    x.reveal();
                }
                self.hand.push(x)
            }
            _ => {}
        }
    }

    pub fn play_card(&mut self, card: usize, target: Target) -> Option<CardHolder> {
        match target {
            Target::Friendly(x) => match x {
                TargetType::Hero => None,
                TargetType::Monster(i) => {
                    let field_slot = self.field[i].clone();
                    match field_slot {
                        None => {
                            if let CardHolder::MonsterCard(chosen_card) = &self.hand.remove(card) {
                                chosen_card.play_card_on_field(self, i);
                            }
                            None
                        }
                        Some(_) => None,
                    }
                }
            },
            _ => None,
        }
    }

    pub fn count_deck_size(&self) -> usize {
        self.deck.len()
    }
}

#[derive(Component)]
pub struct Game {
    players: (Board, Board),
    turns: u32,
}

impl Game {
    pub fn new(player1: Board, player2: Board) -> Self {
        Game {
            players: (player1, player2),
            turns: 0,
        }
    }

    pub fn players(&self) -> &(Board, Board) {
        &self.players
    }
}

pub struct Turn {
    player: u8,
    phase: TurnPhase,
}

pub enum TurnPhase {
    PrePlay,
    InPlay,
    PostPlay,
}
