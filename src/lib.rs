// Insert modules, plugins, and game state plugin
//
//

mod loading;
mod menu;
mod levels;
mod game;
// mod audio;

use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::levels::LevelsPlugin;
use crate::game::CorePlugin;
// use crate::audio::InternalAudioPlugin;

use bevy::app::App;
use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

// Using state to separate gameplay logic from loading logic
// Comprises initial load, game, and menu/waiting
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the main loading state, LoadingPlugin will load the assets
    #[default]
    Loading,
    // Main menu screen and level select
    Menu,
    // During this state the gameplay logic is executed
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            LevelsPlugin,
            CorePlugin,
            // InternalAudioPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default()
            ));
        }
    }
}

// // Animates all sprites including updating the timer and increasing texture atlas
// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
// ) {
//     for (indices, mut timer, mut atlas) in &mut query {
//         if timer.tick(time.delta()).just_finished() {
//             atlas.index = if atlas.index == indices.last {
//                 indices.first
//             } else {
//                 atlas.index + 1
//             }
//         }
//     }
// }
//
// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
// ) {
//     let texture = asset_server.load("textures/")
// }
