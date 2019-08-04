use crate::controls::Controls;
use game::{Game, ToRender};
use prototty::event_routine::common_event::*;
use prototty::event_routine::*;
use prototty::input::*;
use prototty::render::*;
use prototty_storage::Storage;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct GameView;

impl<'a> View<&'a Game> for GameView {
    fn view<F: Frame, C: ColModify>(&mut self, game: &'a Game, context: ViewContext<C>, frame: &mut F) {
        let ToRender { grid } = game.to_render();
        for (coord, cell) in grid.enumerate() {
            let character = match cell.occupant {
                None => '.',
                Some(game::Occupant::Player) => '@',
                Some(game::Occupant::Wall) => '#',
            };
            let view_cell = ViewCell::new().with_character(character);
            frame.set_cell_relative(coord, 0, view_cell, context);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GameInstance {
    game: Game,
}

impl GameInstance {
    fn new() -> Self {
        Self { game: Game::new() }
    }
}

pub struct GameData<S: Storage> {
    instance: Option<GameInstance>,
    controls: Controls,
    _storage: S,
}

impl<S: Storage> GameData<S> {
    pub fn new(controls: Controls, storage: S) -> Self {
        Self {
            instance: None,
            controls,
            _storage: storage,
        }
    }
    pub fn has_instance(&self) -> bool {
        self.instance.is_some()
    }
    pub fn instantiate(&mut self) {
        self.instance = Some(GameInstance::new());
    }
}

pub struct GameEventRoutine<S: Storage>(PhantomData<S>);

impl<S: Storage> GameEventRoutine<S> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub enum GameReturn {
    Pause,
}

impl<S: Storage> EventRoutine for GameEventRoutine<S> {
    type Return = GameReturn;
    type Data = GameData<S>;
    type View = GameView;
    type Event = CommonEvent;

    fn handle<EP>(self, data: &mut Self::Data, _view: &Self::View, event_or_peek: EP) -> Handled<Self::Return, Self>
    where
        EP: EventOrPeek<Event = Self::Event>,
    {
        event_or_peek_with_handled(event_or_peek, self, |s, event| match event {
            CommonEvent::Input(input) => {
                if input == Input::Keyboard(keys::ESCAPE) {
                    return Handled::Return(GameReturn::Pause);
                }
                if let Some(instance) = data.instance.as_mut() {
                    let maybe_game_input = data.controls.get(input);
                    if let Some(game_input) = maybe_game_input {
                        instance.game.handle_input(game_input);
                    }
                }
                Handled::Continue(s)
            }
            CommonEvent::Frame(_) => Handled::Continue(s),
        })
    }

    fn view<F, C>(&self, data: &Self::Data, view: &mut Self::View, context: ViewContext<C>, frame: &mut F)
    where
        F: Frame,
        C: ColModify,
    {
        if let Some(instance) = data.instance.as_ref() {
            view.view(&instance.game, context, frame);
        }
    }
}