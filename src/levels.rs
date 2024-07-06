use bevy::prelude::*;
use bevy::ecs::system::SystemId;

use crate::loading::LevelsAssets;

// How many ticks there are in a second
#[derive(Resource)]
pub struct TickSpeed(f32);

// Level choose event, allowing choosing a different level without the menu
// 0 is a magic number meaning no level
#[derive(Event)]
pub struct LevelSelect(usize);

// We choose to keep the level loading system as a resource
#[derive(Resource)]
struct LevelLoadingSystem(SystemId);

impl FromWorld for LevelLoadingSystem {
    fn from_world(world: &mut World) -> Self {
        LevelLoadingSystem(world.register_system(load_level))
    }
}

// Determine what levels it should be
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelSelect>()
            .init_resource::<LevelLoadingSystem>()
            .add_systems(Update, level_listener);
    }
}

fn load_level(
    commands: Commands,
    levels: Res<LevelsAssets>
) {
    start_gametick(&mut commands);
    commands.spawn(SceneBundle {
        scene: levels.level1,
        ..default()
    });
}

// Start game with default speed
fn start_gametick(commands: &mut Commands) {
    commands.insert_resource(TickSpeed(20.))
}

fn cleanup_gametick(mut commands: Commands) {
    commands.remove_resource::<TickSpeed>()
}

