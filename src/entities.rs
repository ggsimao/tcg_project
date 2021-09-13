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
    fn show_on_hand(&self) -> String;
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

    pub fn name(&self) -> String {
        self.name.clone()
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

    pub fn cost(&self) -> i32 {
        self.cost
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }

    pub fn data(&self) -> MonsterData {
        self.data.clone()
    }
}

impl Card for Monster {
    fn play_card_on_field(&self, board: &mut Board, target: usize) {
        board.field[target] = Some(self.clone());
    }

    fn show_on_hand(&self) -> String {
        let mut show_string = String::new();

        let monster_data = &self.data();

        show_string.push_str(&monster_data.name()[..]);
        show_string.push_str(" ");
        show_string.push_str(&self.cost().to_string()[..]);
        show_string.push_str(" ");
        show_string.push_str(&self.damage().to_string()[..]);
        show_string.push_str("DMG ");
        show_string.push_str(&self.health().to_string()[..]);
        show_string.push_str("/");
        show_string.push_str(&monster_data.base_health().to_string()[..]);

        show_string
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

    pub fn field(&self) -> [Option<Monster>; 5] {
        self.field.clone()
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

    pub fn format_field(&self) -> String {
        let mut field_string: String = String::from("[");
        for (i, m) in self.field.iter().enumerate() {
            if i > 0 {
                field_string.push_str("; ");
            }
            match m {
                Some(c) => {
                    let monster_data = c.data();
                    field_string.push_str("(");
                    field_string.push_str(&monster_data.name()[..]);
                    field_string.push_str(" ");
                    field_string.push_str(&c.damage().to_string()[..]);
                    field_string.push_str("DMG ");
                    field_string.push_str(&c.health().to_string()[..]);
                    field_string.push_str("/");
                    field_string.push_str(&monster_data.base_health().to_string()[..]);
                    field_string.push_str(")");
                }
                None => {
                    field_string.push_str("(Empty slot)");
                }
            }
        }
        field_string.push_str("]");
        field_string
    }

    pub fn format_hand(&self) -> String {
        let mut hand_string: String = String::from("[");
        let mut it = 0;
        for c in &self.hand {
            if it > 0 {
                hand_string.push_str("; ");
            }
            hand_string.push_str(&c.show_on_hand()[..])
        }
        hand_string.push_str("]");
        hand_string
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
