// Disable consol on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, asset::AssetMetaCheck};

use tentaclewars3::GamePlugin;

const DIMENSIONS: (usize, usize) = (1920, 1080);

fn main() {
    App::new()
        // .insert_resource(Msaa::Off)
        // .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tentacle War 3".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    prevent_default_event_handling: false,
                    ..Default::default()
                }),
                ..Default::default()
            }))
        .add_plugins(GamePlugin)
        // .add_systems(Startup, set_window_icon)
        .run();
}

// fn set_window_icon(
//     windows: NonSend<WinitWindows>https://bevy-cheatbook.github.io/programming/states.html,
//     primary_window: Query<Entity, With<PrimaryWindow>>,
// ) {
//     let primary_entity = primary_window.single();
//     let Some(primary) = windows.get_window(primary_entity) else { return; };
//     let icon_buf = Cursor::new(include_bytes!(
//         "../build/macos/AppIcon.iconset/icon_256x256.png"
//     ));
//
//     if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
//         let image = image.into_rgba8();
//         let (width, height) = image.dimensions();
//         let rgba = image.into_raw();
//         let icon = Icon::from_rgba(rgba, width, height).unwrap();
//         primary.set_window_icon(Some(icon));
//     }
// }

