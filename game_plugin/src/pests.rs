use std::collections::HashMap;
use bevy::prelude::*;
use anyhow::Result;
use crate::{
    GameState,
    turn_structure::TurnState,
    loading::TextureAssets,
    map::{GameLayer, MAP_SIZE, TilePos, Blocking},
};
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Pest {
    pattern: Vec<IVec2>,
    move_idx: usize,
    ticks_since_move: usize,
    is_blocking: bool,
    sprite: String,
    consumption_layer: GameLayer,
    stop_after_consumption: bool,
}
impl Pest {
    fn spawn(self, position: TilePos, commands: &mut Commands) -> Entity {
        let is_blocking = self.is_blocking;
        let e = commands.spawn()
                .insert(self)
                .insert(IdlePest)
                .insert(position)
                .insert(GameLayer::Pests)
                .id();
        if is_blocking {
            commands.entity(e).insert(Blocking);
        }
        e
    }

    fn rightward_rabbit() -> Self {
        Self {
            pattern: vec![
                IVec2::new(1, 0),
                IVec2::new(1, 0),
                IVec2::new(0, -1),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "rabbit".to_string(),
            is_blocking: true,
            consumption_layer: GameLayer::Plants,
            stop_after_consumption: true,
        }
    }

    fn leftward_rabbit() -> Self {
        Self {
            pattern: vec![
                IVec2::new(-1, 0),
                IVec2::new(-1, 0),
                IVec2::new(0, 1),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "rabbit".to_string(),
            is_blocking: true,
            consumption_layer: GameLayer::Plants,
            stop_after_consumption: true,
        }
    }

    fn upward_rabbit() -> Self {
        Self {
            pattern: vec![
                IVec2::new(0, 1,),
                IVec2::new(0, 1,),
                IVec2::new(1, 0),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "rabbit".to_string(),
            is_blocking: true,
            consumption_layer: GameLayer::Plants,
            stop_after_consumption: true,
        }
    }

    fn downward_rabbit() -> Self {
        Self {
            pattern: vec![
                IVec2::new(0, -1,),
                IVec2::new(0, -1,),
                IVec2::new(-1, 0),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "rabbit".to_string(),
            is_blocking: true,
            consumption_layer: GameLayer::Plants,
            stop_after_consumption: true,
        }
    }

    fn downward_wind() -> Self {
        Self {
            pattern: vec![
                IVec2::new(0, -1,),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "wind".to_string(),
            is_blocking: false,
            consumption_layer: GameLayer::Fences,
            stop_after_consumption: true,
        }
    }

    fn upward_wind() -> Self {
        Self {
            pattern: vec![
                IVec2::new(0, 1,),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "wind".to_string(),
            is_blocking: false,
            consumption_layer: GameLayer::Fences,
            stop_after_consumption: true,
        }
    }

    fn leftward_wind() -> Self {
        Self {
            pattern: vec![
                IVec2::new(-1, 0),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "wind".to_string(),
            is_blocking: false,
            consumption_layer: GameLayer::Fences,
            stop_after_consumption: true,
        }
    }

    fn rightward_wind() -> Self {
        Self {
            pattern: vec![
                IVec2::new(1, 0),
            ],
            move_idx: 0,
            ticks_since_move: 0,
            sprite: "wind".to_string(),
            is_blocking: false,
            consumption_layer: GameLayer::Fences,
            stop_after_consumption: true,
        }
    }
}
pub struct IdlePest;

pub struct PestPlugin;

impl Plugin for PestPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_exit(TurnState::RoundSetup)
                .with_system(move_idle_pests_in.system().label("move idle"))
                .with_system(spawn_pests.system().chain(spawn_tile_sprites.system()).after("move idle"))
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::RoundCleanup)
                .with_system(despawn_pests.system())
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::PestTurnA)
                .with_system(pest_movement.system())
                .with_system(maybe_end_pest_turn.system())
        );
    }
}

fn pest_movement(
    mut commands: Commands,
    mut queries: QuerySet<(
        Query<(Entity, &TilePos, &GameLayer), Without<Blocking>>,
        Query<(Entity, &mut Pest, &mut TilePos, &GameLayer), Without<IdlePest>>
    )>,
) {
    let mut current_positions = HashMap::with_capacity(10);
    for (e, pos, layer) in queries.q0().iter() {
        current_positions.insert(pos.0, (e, *layer));
    }
    for (e, mut pest, mut pos, layer) in queries.q1_mut().iter_mut() {
        let movement = pest.pattern[pest.move_idx];
        pest.move_idx = (pest.move_idx + 1) % pest.pattern.len();
        let new_pos = pos.0 + movement;
        if new_pos.x >= 0 && new_pos.y >= 0 && new_pos.x < MAP_SIZE as i32 && new_pos.y < MAP_SIZE as i32 {
            let mut did_move = !pest.is_blocking;
            if let Some((other, layer)) = current_positions.get(&new_pos) {
                if layer == &pest.consumption_layer {
                    did_move = true;
                    commands.entity(*other).despawn_recursive();
                    if pest.stop_after_consumption {
                        commands.entity(e).despawn_recursive();
                    }
                }
            } else {
                did_move = true;
            }
            if !did_move {
                pest.ticks_since_move += 1;
            } else {
                current_positions.remove(&new_pos);
                current_positions.insert(new_pos, (e, *layer));
                *pos = TilePos(new_pos);
            }
        } else {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn maybe_end_pest_turn(
    mut state: ResMut<State<TurnState>>,
    mut pest_query: Query<&Pest, Without<IdlePest>>,
) {
    for pest in pest_query.iter() {
        if pest.ticks_since_move < 3 {
            return
        }
    }
    // If we've reached this point then every pest on the board
    // has been unable to move for a full cycle of their motion
    // pattern which means they are all permanently blocked and
    // the round can end.
    state.set(TurnState::EndOfRound);
}

fn move_idle_pests_in(
    mut commands: Commands,
    mut pest_query: Query<(Entity, &mut TilePos), With<IdlePest>>,
) {
    for (e, mut pos) in pest_query.iter_mut() {
        if pos.0.x == 0 {
            pos.0.x += 1;
        } else if pos.0.x == MAP_SIZE-1 {
            pos.0.x -= 1;
        } else if pos.0.y == 0 {
            pos.0.y += 1;
        } else if pos.0.y == MAP_SIZE-1 {
            pos.0.y -= 1;
        }
        commands.entity(e).remove::<IdlePest>();
    }
}

fn spawn_pests(
    mut commands: Commands,
) -> Result<Vec<(Entity, String)>> {
    let mut rng = thread_rng();
    let mut spawn_slots = vec![];
    for i in 3..MAP_SIZE-2 {
        spawn_slots.push((0, i));
        spawn_slots.push((MAP_SIZE-1, i));
        spawn_slots.push((i, 0));
        spawn_slots.push((i, MAP_SIZE-1));
    }
    let count = rng.gen_range(1..4);
    let mut spawned = Vec::with_capacity(count);
    for (x, y) in spawn_slots.choose_multiple(&mut rng, count) {
        let mut position = TilePos(IVec2::new(*x,*y));
        let pest = if *x == 0 {
            if rng.gen::<f32>() > 0.1 {
                Pest::rightward_rabbit()
            } else {
                Pest::rightward_wind()
            }
        } else if *x == MAP_SIZE-1 {
            if rng.gen::<f32>() > 0.1 {
                Pest::leftward_rabbit()
            } else {
                Pest::leftward_wind()
            }
        } else if *y == 0 {
            if rng.gen::<f32>() > 0.1 {
                Pest::upward_rabbit()
            } else {
                Pest::upward_wind()
            }
        } else {
            if rng.gen::<f32>() > 0.1 {
                Pest::downward_rabbit()
            } else {
                Pest::downward_wind()
            }
        };
        //FIXME: This is dumb and tangled from too much fiddling
        let sprite = pest.sprite.clone();
        spawned.push((pest.spawn(position, &mut commands), sprite));
    }
    Ok(spawned)
}

fn despawn_pests(
    mut commands: Commands,
    mut pest_query: Query<Entity, (With<Pest>, Without<IdlePest>)>,
) {
    for e in pest_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn spawn_tile_sprites(
    In(to_spawn): In<Result<Vec<(Entity, String)>>>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(to_spawn) = to_spawn {
        for (e, sprite) in &to_spawn {
            let handle = match sprite.as_str() {
                "rabbit" => textures.rabbit.clone(),
                "wind" => textures.wind.clone(),
                _ => unimplemented!()
            };
            commands.entity(*e).insert_bundle(SpriteBundle {
                material: materials.add(handle.into()),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            });
        }
    }
}

