use bevy::{
    prelude::*,
    input::{
        keyboard::KeyboardInput,
        ElementState,
    }
};
use crate::GameState;

pub struct TurnPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum TurnState {
    Idle,
    RoundSetup,
    StartOfRound,
    PlayerTurn,
    PestTurn,
    EndOfRound,
    RoundCleanup,
}

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(TurnState::Idle)
           .add_system_set(
               SystemSet::on_update(GameState::Playing)
                   .with_system(progress_turn.system())
           );
    }
}

fn progress_turn(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut state: ResMut<State<TurnState>>,
) {
    // These outer states should be moved through as quickly as possible regardless of player interaction
    match state.current() {
        TurnState::Idle => state.set(TurnState::RoundSetup).unwrap(),
        TurnState::RoundSetup => state.set(TurnState::StartOfRound).unwrap(),
        TurnState::StartOfRound => state.set(TurnState::PlayerTurn).unwrap(),
        _ => {
            // These inner states should only move forward when the player takes the action necessary to end their turn
            for event in keyboard_input_events.iter() {
                if event.state == ElementState::Pressed && event.key_code == Some(KeyCode::Space) {
                match state.current() {
                        TurnState::PlayerTurn => state.set(TurnState::PestTurn).unwrap(),
                        TurnState::PestTurn=> state.set(TurnState::PlayerTurn).unwrap(),
                        _ => ()
                    }
                }
            }
        }
    }
}
