use std::ops::Deref;
use std::fmt;

use bevy::prelude::*;
use bevy::scene::SceneInstance;
use bevy::ecs::system::SystemId;

use crate::loading::LevelsAssets;
use crate::GameState;

// How many ticks there are in a second
#[derive(Resource)]
pub struct TickSpeed(f32);

// The current level which we load when entering GameState::Playing
#[derive(Resource, Default, Debug)]
pub struct CurrentLevel(usize);

impl fmt::Display for CurrentLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Level choose event, allowing choosing a different level without the menu
// 0 is a magic number meaning no level
#[derive(Event)]
pub struct LevelSelect(usize);

// Determine what levels it should be
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelSelect>()
            .init_resource::<CurrentLevel>()
            .add_systems(OnEnter(GameState::Playing), load_level)
            .add_systems(OnExit(GameState::Playing), cleanup_active_levels);
    }
}

fn load_level(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    levels: Res<LevelsAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // We cannot continue without a level selected
    if current_level.0 == 0 { 
        warn!("No level selected; returning to menu");
        next_state.set(GameState::Menu);
    }

    // Obtain the particular level determined by current_level
    let level = levels
        .into_inner()
        .field(&("level".to_owned() + &current_level.to_string()));
    let Some(level) = level else {
        error!("Invalid level ${:?}", current_level);
        return;
    };

    start_gametick(&mut commands);
    commands.spawn(SceneBundle {
        scene: level.downcast_ref::<Handle<Scene>>().unwrap().clone(),
        ..default()
    });
}

// Cleans up active levels (which have the SceneInstance component by def)
fn cleanup_active_levels(query: Query<Entity, With<SceneInstance>>, mut commands: Commands) {
    for scene in &query {
        commands.entity(scene).despawn_recursive();
    }
}

// Start game with default speed
fn start_gametick(commands: &mut Commands) {
    commands.insert_resource(TickSpeed(20.))
}

fn cleanup_gametick(mut commands: Commands) {
    commands.remove_resource::<TickSpeed>()
}

