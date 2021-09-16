use rltk::{GameState, Point, Rltk};
use specs::prelude::*;

use entities::{Board, Hero, Monster};

mod card_data;
mod entities;
mod gui;
mod match_cycle;

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
    fn tick(&mut self, ctx: &mut Rltk) {}
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

    // gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

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
