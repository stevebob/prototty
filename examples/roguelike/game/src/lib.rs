pub use direction::CardinalDirection;
pub use grid_2d::{Coord, Grid, Size};
use rand::{Rng, SeedableRng};
use rand_isaac::Isaac64Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod entity_grid;
mod realtime_gas;
mod realtime_projectile;
mod world;

use world::{CharacterId, World};
pub use world::{CharacterTile, ProjectileTile, ToRender, WallTile};

#[derive(Clone, Copy)]
pub enum Input {
    Walk(CardinalDirection),
    Fire(Coord),
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    world: World,
    player_id: CharacterId,
    rng: Isaac64Rng,
}

impl Game {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let s = include_str!("terrain.txt");
        let rows = s.split("\n").filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let size = Size::new_u16(rows[0].len() as u16, rows.len() as u16);
        let mut world = World::new(size);
        let mut player_id = None;
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                let coord = Coord::new(x as i32, y as i32);
                match ch {
                    '.' => (),
                    '#' => world.spawn_wall(coord),
                    '@' => {
                        player_id = Some(world.spawn_player(coord));
                    }
                    _ => panic!("unexpected char: {}", ch),
                }
            }
        }
        Self {
            world,
            player_id: player_id.expect("didn't create player"),
            rng: Isaac64Rng::seed_from_u64(rng.gen()),
        }
    }
    pub fn has_animations(&self) -> bool {
        self.world.has_pending_animations()
    }
    pub fn handle_input(&mut self, input: Input) {
        if !self.has_animations() {
            match input {
                Input::Walk(direction) => self.world.character_walk_in_direction(self.player_id, direction),
                Input::Fire(coord) => self.world.character_fire_bullet(self.player_id, coord),
            }
        }
    }
    pub fn handle_tick(&mut self, _since_last_tick: Duration) {
        self.world.animation_tick();
    }
    pub fn player_coord(&self) -> Coord {
        self.world.character_coord(self.player_id)
    }
    pub fn world_size(&self) -> Size {
        self.world.size()
    }
    pub fn world_to_render(&self) -> ToRender {
        self.world.to_render()
    }
}
