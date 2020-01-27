use crate::{
    world::{
        data::Components,
        realtime_periodic::{
            core::{RealtimePeriodicState, TimeConsumingEvent},
            movement::MovementState,
            particle::ParticleEmitterState,
        },
        spatial_grid::SpatialGrid,
    },
    ExternalEvent,
};
use ecs::{Ecs, Entity};
use rand::Rng;
use rgb24::Rgb24;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / 60);

pub fn period_per_frame(num_per_frame: u32) -> Duration {
    FRAME_DURATION / num_per_frame
}

crate::realtime_periodic! {
    realtime_periodic {
        movement: MovementState,
        particle_emitter: ParticleEmitterState,
        fade: FadeState,
        light_colour_fade: LightColourFadeState,
    }
}

pub use realtime_periodic::RealtimeComponents;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FadeProgress {
    Fading(u8),
    Complete,
}

impl FadeProgress {
    fn fading(self) -> Option<u8> {
        match self {
            Self::Fading(progress) => Some(progress),
            Self::Complete => None,
        }
    }
    fn is_complete(self) -> bool {
        match self {
            Self::Fading(_) => false,
            Self::Complete => true,
        }
    }
}

impl Default for FadeProgress {
    fn default() -> Self {
        Self::Fading(0)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FadeState {
    progress: FadeProgress,
    period: Duration,
}

impl FadeState {
    pub fn new(duration: Duration) -> Self {
        Self::new_with_progress(duration, FadeProgress::default())
    }
    pub fn new_with_progress(full_duration: Duration, progress: FadeProgress) -> Self {
        let period = full_duration / 256;
        Self { progress, period }
    }
    pub fn fading(self) -> Option<u8> {
        self.progress.fading()
    }
}

impl RealtimePeriodicState for FadeState {
    type Event = FadeProgress;
    type Components = RealtimeComponents;
    fn tick<R: Rng>(&mut self, _rng: &mut R) -> TimeConsumingEvent<Self::Event> {
        self.progress = match self.progress {
            FadeProgress::Complete => FadeProgress::Complete,
            FadeProgress::Fading(progress) => match progress.checked_add(1) {
                Some(progress) => FadeProgress::Fading(progress),
                None => FadeProgress::Complete,
            },
        };
        TimeConsumingEvent {
            event: self.progress,
            until_next_event: self.period,
        }
    }
    fn animate_event(
        progress: Self::Event,
        ecs: &mut Ecs<Components>,
        _realtime_components: &mut Self::Components,
        _spatial_grid: &mut SpatialGrid,
        entity: Entity,
        _external_events: &mut Vec<ExternalEvent>,
    ) {
        if progress.is_complete() {
            ecs.remove(entity);
        }
    }
}

pub enum LightColourFadeProgress {
    Colour(Rgb24),
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightColourFadeState {
    pub fade_state: FadeState,
    pub from: Rgb24,
    pub to: Rgb24,
}

impl RealtimePeriodicState for LightColourFadeState {
    type Event = LightColourFadeProgress;
    type Components = RealtimeComponents;
    fn tick<R: Rng>(&mut self, rng: &mut R) -> TimeConsumingEvent<Self::Event> {
        let TimeConsumingEvent {
            event: fade_progress,
            until_next_event,
        } = self.fade_state.tick(rng);
        let event = match fade_progress {
            FadeProgress::Complete => LightColourFadeProgress::Complete,
            FadeProgress::Fading(fading) => {
                LightColourFadeProgress::Colour(self.from.linear_interpolate(self.to, fading))
            }
        };
        TimeConsumingEvent {
            event,
            until_next_event,
        }
    }
    fn animate_event(
        progress: Self::Event,
        ecs: &mut Ecs<Components>,
        _realtime_components: &mut Self::Components,
        _spatial_grid: &mut SpatialGrid,
        entity: Entity,
        _external_events: &mut Vec<ExternalEvent>,
    ) {
        match progress {
            LightColourFadeProgress::Colour(colour) => {
                if let Some(light) = ecs.components.light.get_mut(entity) {
                    light.colour = colour;
                }
            }
            LightColourFadeProgress::Complete => {
                ecs.components.light.remove(entity);
            }
        }
    }
}