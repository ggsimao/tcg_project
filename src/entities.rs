enum HeroClass {
    Mage,
    Warrior,
    Ranger,
    Rogue,
    Priest,
}

enum DamageType {
    Physical(i32),
    Magical(MagicSchool),
}

enum MagicSchool {
    Flame,
    Frost,
    Lightning,
    Shadow,
    Light,
}

trait Card {
    
}

trait Effect {
    
}

struct MonsterData {
    name: String,
    base_cost: i32,
    base_health: i32,
    base_damage: i32,
}

struct Monster {
    cost: i32,
    health: i32,
    damage: i32,
    data: MonsterData,
}

struct Magic {
    cost: i32,
    effect: Box<dyn FnMut()>,
}

struct Hero {
    health: i32,
    class: HeroClass,
}

struct Board {
    hero: Hero,
    field: [Option<Monster>; 5],
    hand: Vec<Box<dyn Card>>,
    deck: Vec<Box<dyn Card>>,
    graveyard: Vec<Box<dyn Card>>,
}

struct Turn {
    player: u8,
    phase: TurnPhase,
}

enum TurnPhase {
    PrePlay,
    InPlay,
    PostPlay,
}