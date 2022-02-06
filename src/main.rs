use rand::prelude::*;
use rltk::{GameState, Point, Rltk};
use specs::prelude::*;

use entities::{Board, CardHolder, Game, Hero, HeroClass, Monster, MonsterData};

mod card_data;
mod entities;
mod gui;
mod match_cycle;
mod player_system;

pub enum Command {
    ChooseSource(String),
    ChooseTarget(String),
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        // let mut vis = VisibilitySystem {};
        // vis.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        gui::draw_empty_board(&self.ecs, ctx, 0);
        gui::draw_empty_board(&self.ecs, ctx, 1);
        gui::draw_filled_board(&self.ecs, ctx);
        gui::display_hand(&self.ecs, ctx);

        player_system::player_input(&mut self.ecs, ctx);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let mut context = RltkBuilder::simple(81, 80)
        .unwrap()
        .with_title("TCG PROJECT")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Monster>();
    gs.ecs.register::<Board>();
    gs.ecs.register::<Hero>();
    // gs.ecs.register::<SimpleMarker<SerializeMe>>();
    // gs.ecs.register::<SerializationHelper>();

    card_data::initialize_card_data(&mut gs.ecs);
    let hero1 = Hero::new(0, 30, 30, HeroClass::Mage);
    let hero2 = Hero::new(0, 30, 30, HeroClass::Mage);
    let mut deck1 = vec![];
    let mut deck2 = vec![];
    {
        let monster_data = gs.ecs.entry::<Vec<MonsterData>>().or_insert(vec![]);

        // gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
        let mut id = 0;
        {
            for _ in 0..30 {
                let orc1 = CardHolder::MonsterCard(Monster::new(id, (&monster_data[0]).clone()));
                let orc2 = CardHolder::MonsterCard(Monster::new(id, (&monster_data[0]).clone()));
                let wisp1 =
                    CardHolder::MonsterCard(Monster::new(id + 30, (&monster_data[1]).clone()));
                let wisp2 =
                    CardHolder::MonsterCard(Monster::new(id + 30, (&monster_data[1]).clone()));

                let mut rng = rand::thread_rng();

                let n1: u8 = rng.gen();
                let n2: u8 = rng.gen();

                if n1 % 2 == 0 {
                    deck1.push(orc1);
                } else {
                    deck1.push(wisp1);
                }
                if n2 % 2 == 0 {
                    deck2.push(orc2);
                } else {
                    deck2.push(wisp2);
                }

                id += 1;
            }
        }
    }
    let board1 = Board::new(0, hero1, deck1);
    let board2 = Board::new(1, hero2, deck2);
    {
        gs.ecs.create_entity().with(board1).build();
    }
    {
        gs.ecs.create_entity().with(board2).build();
    }
    match_cycle::pre_game(&gs.ecs);
    // gs.ecs.insert(Game::new(board1, board2));

    // let map: Map = Map::new_map_rooms_and_corridors();
    // let (player_x, player_y) = map.rooms[0].center();

    // let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // gs.ecs.insert(rltk::RandomNumberGenerator::new());
    // for room in map.rooms.iter().skip(1) {
    //     spawner::spawn_room(&mut gs.ecs, room);
    // }

    // gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
