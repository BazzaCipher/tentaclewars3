use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
                .load_collection::<MenusAssets>()
                .load_collection::<LevelsAssets>()
            );
    }
}

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/background.ogg")]
    background: Handle<AudioSource>
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "images/tentacle.png")]
    pub tentacle: Handle<Image>
}

// Main menu, other menus, etc, are all scenes (groups of entities)
// These are where the levels are able to be accessed. Because
// a hashmap cannot be a resource, I have resorted to the explicit promise
// of each level being named `level${a}` where a is a number from 1 of usize
#[derive(AssetCollection, Resource)]
pub struct LevelsAssets {
    #[asset(path = "scenes/level1.scn.ron")]
    level1: Handle<Scene>,
    #[asset(path = "scenes/level2.scn.ron")]
    level2: Handle<Scene>,
}

// Menu scenes, to be applied individually or on top
#[derive(AssetCollection, Resource)]
pub struct MenusAssets {

}

fn use_asset_handles(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.background.clone(),
        ..default()
    });
}

