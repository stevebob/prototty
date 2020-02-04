use crate::{visibility::Light, ExternalEvent};
use ecs::{Ecs, Entity};
use grid_2d::{Coord, Grid, Size};
use rand::Rng;
use rgb24::Rgb24;
use serde::{Deserialize, Serialize};

mod spatial_grid;
use spatial_grid::SpatialGrid;

mod data;
use data::{Components, Npc};
pub use data::{Disposition, Layer, Tile};

mod realtime_periodic;
pub use realtime_periodic::animation::Context as AnimationContext;
use realtime_periodic::data::RealtimeComponents;

mod query;
pub use query::WorldQuery;

mod explosion;
pub use explosion::spec as explosion_spec;

mod action;
pub use action::WorldAction;

mod spawn;
pub use spawn::WorldSpawn;

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub ecs: Ecs<Components>,
    pub realtime_components: RealtimeComponents,
    pub spatial_grid: SpatialGrid,
}

impl World {
    pub fn new(size: Size) -> Self {
        let ecs = Ecs::new();
        let realtime_components = RealtimeComponents::default();
        let spatial_grid = Grid::new_default(size);
        Self {
            ecs,
            realtime_components,
            spatial_grid,
        }
    }
}

impl World {
    pub fn to_render_entities<'a>(&'a self) -> impl 'a + Iterator<Item = ToRenderEntity> {
        let tile_component = &self.ecs.components.tile;
        let location_component = &self.ecs.components.location;
        let realtime_fade_component = &self.realtime_components.fade;
        let colour_hint_component = &self.ecs.components.colour_hint;
        let blood_component = &self.ecs.components.blood;
        tile_component.iter().filter_map(move |(entity, &tile)| {
            if let Some(location) = location_component.get(entity) {
                let fade = realtime_fade_component.get(entity).and_then(|f| f.state.fading());
                let colour_hint = colour_hint_component.get(entity).cloned();
                let blood = blood_component.contains(entity);
                Some(ToRenderEntity {
                    coord: location.coord,
                    layer: location.layer,
                    tile,
                    fade,
                    colour_hint,
                    blood,
                })
            } else {
                None
            }
        })
    }

    pub fn all_lights_by_coord<'a>(&'a self) -> impl 'a + Iterator<Item = (Coord, &'a Light)> {
        self.ecs.components.light.iter().filter_map(move |(entity, light)| {
            self.ecs
                .components
                .location
                .get(entity)
                .map(|location| (location.coord, light))
        })
    }
}

impl World {
    pub fn entity_coord(&self, entity: Entity) -> Option<Coord> {
        self.ecs.components.location.get(entity).map(|l| l.coord)
    }
    pub fn entity_npc(&self, entity: Entity) -> &Npc {
        self.ecs.components.npc.get(entity).unwrap()
    }
    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.ecs.entity_allocator.exists(entity)
    }
    pub fn size(&self) -> Size {
        self.spatial_grid.size()
    }
    pub fn is_gameplay_blocked(&self) -> bool {
        !self.ecs.components.blocks_gameplay.is_empty()
    }
    pub fn animation_tick<R: Rng>(
        &mut self,
        animation_context: &mut AnimationContext,
        external_events: &mut Vec<ExternalEvent>,
        rng: &mut R,
    ) {
        animation_context.tick(self, external_events, rng)
    }
}

pub struct ToRenderEntity {
    pub coord: Coord,
    pub layer: Option<Layer>,
    pub tile: Tile,
    pub fade: Option<u8>,
    pub colour_hint: Option<Rgb24>,
    pub blood: bool,
}
