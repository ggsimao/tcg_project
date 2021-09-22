use specs::prelude::*;
use specs_derive::*;
use std::any;

pub enum HeroClass {
    Mage,
    Warrior,
    Ranger,
    Rogue,
    Priest,
}

pub enum Target {
    Friendly(TargetType),
    Enemy(TargetType),
}

pub enum TargetType {
    Hero,
    Monster(usize),
}

#[derive(Copy, Clone)]
pub enum DamageType {
    Physical(i32),
    Magical(MagicSchool),
}

#[derive(Copy, Clone)]
pub enum MagicSchool {
    Flame,
    Frost,
    Lightning,
    Shadow,
    Light,
}

pub trait Card {
    fn play_card_on_field(&self, board: &mut Board, target: usize);
    fn play_card_on_hero(&self, board: &mut Board) {}
    fn data(&self) -> Box<&dyn CardData>;
    fn id(&self) -> u32;
}

pub trait CardData {
    fn name(&self) -> String {
        String::new()
    }
    fn as_monster(&self) -> Option<&MonsterData>;
}

pub trait Effect {}

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
}

impl Monster {
    pub fn new(id: u32, cost: i32, health: i32, damage: i32, data: MonsterData) -> Monster {
        Monster {
            id: id,
            cost: cost,
            health: health,
            damage: damage,
            data: data,
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
}

pub struct Magic {
    cost: i32,
    effect: Box<dyn FnMut()>,
}

#[derive(Component)]
pub struct Hero {
    id: u8,
    base_health: i32,
    health: i32,
    class: HeroClass,
}

impl Hero {
    fn id(&self) -> u8 {
        self.id
    }
}

#[derive(Component)]
pub struct Board {
    id: u8,
    hero: Hero,
    field: [Option<Monster>; 5],
    hand: Vec<Box<dyn Card + Send + Sync>>,
    deck: Vec<Box<dyn Card + Send + Sync>>,
    graveyard: Vec<Box<dyn Card + Send + Sync>>,
    highlighted: (u8, i32),
}

impl Board {
    pub fn new(
        id: u8,
        hero: Hero,
        field: [Option<Monster>; 5],
        hand: Vec<Box<dyn Card + Send + Sync>>,
        deck: Vec<Box<dyn Card + Send + Sync>>,
        graveyard: Vec<Box<dyn Card + Send + Sync>>,
    ) -> Board {
        Board {
            id: id,
            hero: hero,
            field: field,
            hand: hand,
            deck: deck,
            graveyard: graveyard,
            highlighted: (0, -1),
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn field(&self) -> [Option<Monster>; 5] {
        self.field.clone()
    }

    pub fn hand(&self) -> &Vec<Box<dyn Card + Send + Sync>> {
        &self.hand
    }

    pub fn highlighted(&self) -> (u8, i32) {
        self.highlighted
    }

    pub fn draw_card(&mut self) {
        match self.deck.pop() {
            Some(x) => self.hand.push(x),
            _ => {}
        }
    }

    pub fn play_card(&mut self, card: usize, target: Target) -> Option<Box<dyn Card>> {
        match target {
            Target::Friendly(x) => match x {
                TargetType::Hero => None,
                TargetType::Monster(i) => {
                    let field_slot = self.field[i].clone();
                    match field_slot {
                        None => {
                            let chosen_card = &self.hand.remove(card);
                            chosen_card.play_card_on_field(self, i);
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

pub struct Turn {
    player: u8,
    phase: TurnPhase,
}

pub enum TurnPhase {
    PrePlay,
    InPlay,
    PostPlay,
}
