use bevy::prelude::*;
use crate::{
    GameState,
    loading::TextureAssets,
    turn_structure::TurnState,
    main_ui::{despawn_overlay, GameOverlay},
    plants::{Plant, RoundsTillMature},
};

pub struct ScoringPlugin;
#[derive(Default)]
pub struct PrizePlantScore(pub u32);
struct PrizeScreenTimer(Timer);

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PrizePlantScore>();
        app.insert_resource(PrizeScreenTimer(Timer::from_seconds(2.0, true)));
        app.add_system_set(
            SystemSet::on_enter(TurnState::EndOfRound)
                .with_system(score.system().after("incr_maturity"))
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::PrizePlantScoring)
                .with_system(score_prize_plant.system().after("incr_maturity"))
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::PrizePlantScoring)
                .with_system(despawn_overlay.system())
        );
        app.add_system_set(
            SystemSet::on_update(GameState::PrizePlantScoring)
                .with_system(score_screen_timer.system())
        );
    }
}

fn score(
    plant_query: Query<(&Plant, &RoundsTillMature)>,
) {
    let mut score = 0;
    for (plant, rounds_till_mature) in plant_query.iter() {
        if rounds_till_mature.0 == 0 {
            score += plant.0;
        }
    }
    println!("Round score: {}", score);
}

fn score_screen_timer(
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut prize_screen_timer: ResMut<PrizeScreenTimer>,
) {
    prize_screen_timer.0.tick(time.delta());
    if prize_screen_timer.0.just_finished() {
        state.set(GameState::Menu);
    }
}

fn score_prize_plant(
    mut commands: Commands,
    score: Res<PrizePlantScore>,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.overlay.clone().into()),
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..Default::default()
    }).insert(GameOverlay);
    if score.0 == 0 {
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(textures.no_prize.clone().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }).insert(GameOverlay);
    } else if score.0 == 10 {
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(textures.first_prize.clone().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }).insert(GameOverlay);
    } else if score.0 > 5 {
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(textures.second_prize.clone().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }).insert(GameOverlay);
    } else {
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(textures.third_prize.clone().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }).insert(GameOverlay);
    }

    println!("Prize Plant Score: {}", score.0);
}
