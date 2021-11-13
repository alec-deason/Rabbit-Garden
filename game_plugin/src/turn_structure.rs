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
    PestTurnA,
    PestTurnB,
    EndOfRound,
    RoundCleanup,
}

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(TurnState::Idle)
           .add_system_set(
               SystemSet::on_update(GameState::Playing)
                   .with_system(progress_turn.system())
           )
           .add_system_set(
               SystemSet::on_exit(GameState::Playing)
                   .with_system(reset_turn_state.system())
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
        TurnState::EndOfRound => state.set(TurnState::RoundCleanup).unwrap(),
        TurnState::RoundCleanup => state.set(TurnState::RoundSetup).unwrap(),
        TurnState::PestTurnA => state.set(TurnState::PestTurnB).unwrap(),
        TurnState::PestTurnB => state.set(TurnState::PestTurnA).unwrap(),
        _ => {
            // These inner states should only move forward when the player takes the action necessary to end their turn
            for event in keyboard_input_events.iter() {
                if event.state == ElementState::Pressed && event.key_code == Some(KeyCode::Space) {
                match state.current() {
                        TurnState::PlayerTurn => state.set(TurnState::PestTurnA).unwrap(),
                        _ => ()
                    }
                }
            }
        }
    }
}

fn reset_turn_state(
    mut state: ResMut<State<TurnState>>,
) {
    state.set(TurnState::Idle);
}
