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
            .insert_resource(TurnTimer(Timer::from_seconds(0.05, true)))
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

struct TurnTimer(Timer);

fn progress_turn(
    time: Res<Time>,
    mut turn_timer: ResMut<TurnTimer>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut state: ResMut<State<TurnState>>,
) {
    turn_timer.0.tick(time.delta());
    match state.current() {
        TurnState::Idle => state.set(TurnState::RoundSetup).unwrap(),
        TurnState::RoundSetup => state.set(TurnState::StartOfRound).unwrap(),
        TurnState::StartOfRound => state.set(TurnState::PlayerTurn).unwrap(),
        TurnState::EndOfRound => state.set(TurnState::RoundCleanup).unwrap(),
        TurnState::RoundCleanup => state.set(TurnState::RoundSetup).unwrap(),
        TurnState::PestTurnA => {
            if turn_timer.0.just_finished() {
                state.set(TurnState::PestTurnB).unwrap()
            }
        }
        TurnState::PestTurnB => {
            if turn_timer.0.just_finished() {
                state.set(TurnState::PestTurnA).unwrap()
            }
        }
        _ => ()
    }
}

fn reset_turn_state(
    mut state: ResMut<State<TurnState>>,
) {
    state.set(TurnState::Idle);
}
