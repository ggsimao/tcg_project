use std::fmt;

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
}

#[derive(Clone)]
pub struct Monster {
    cost: i32,
    health: i32,
    damage: i32,
    data: MonsterData,
}

impl Monster {
    pub fn new(cost: i32, health: i32, damage: i32, data: MonsterData) -> Monster {
        Monster {
            cost: cost,
            health: health,
            damage: damage,
            data: data,
        }
    }
}

impl Card for Monster {
    fn play_card_on_field(&self, board: &mut Board, target: usize) {
        board.field[target] = Some(self.clone());
    }
}

pub struct Magic {
    cost: i32,
    effect: Box<dyn FnMut()>,
}

pub struct Hero {
    base_health: i32,
    health: i32,
    class: HeroClass,
}

pub struct Board {
    hero: Hero,
    field: [Option<Monster>; 5],
    hand: Vec<Box<dyn Card>>,
    deck: Vec<Box<dyn Card>>,
    graveyard: Vec<Box<dyn Card>>,
}

impl Board {
    pub fn new(
        hero: Hero,
        field: [Option<Monster>; 5],
        hand: Vec<Box<dyn Card>>,
        deck: Vec<Box<dyn Card>>,
        graveyard: Vec<Box<dyn Card>>,
    ) -> Board {
        Board {
            hero: hero,
            field: field,
            hand: hand,
            deck: deck,
            graveyard: graveyard,
        }
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
