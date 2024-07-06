use crate::GameState;
use crate::loading::ImageAssets;

use bevy::prelude::*;
use webbrowser;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Hand off is from the LoadingPlugin
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_menu_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

// Marker trait for every menu component
#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands, textures: Res<ImageAssets>) {
    info!("menu");
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        Menu
    ))
    .with_children(|children| {
        let button_kind = BasicButtons::MenuButton;
        children.spawn((
            ButtonBundle::from(&button_kind),
            ButtonColors::from(&button_kind),
            ButtonAction::ChangeGameState(GameState::Playing)
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle { 
                    font_size: 40., // Maybe percentage? 
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                }
            ));
        });
    });
    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        Menu
    ))
    .with_children(|children| {
        let mut button_colors: ButtonColors = (&BasicButtons::MenuButton).into();
        button_colors.normal = Color::NONE;
        children.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(170.),
                    height: Val::Px(50.),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                background_color: button_colors.normal.into(),
                ..default()
            },
            button_colors,
            ButtonAction::OpenWebLink("https://bazza.space")
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Made by Barry on Bevy",
                TextStyle {
                    font_size: 15.,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
            parent.spawn(ImageBundle {
                image: textures.bevy.clone().into(),
                style: Style {
                    width: Val::Px(32.),
                    ..default()
                },
                ..default()
            });
        });
    });
}

// Struct for choosing button colors
#[derive(Copy, Clone, Default, Component)]
pub struct ButtonColors {
    normal: Color,
    hovered: Color,
    border: Option<Color>,
    border_hovered: Option<Color>,
}

/// Templates for buttons for convenience
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum BasicButtons {
    SmallButton,
    MenuButton,

}

impl From<&BasicButtons> for ButtonColors {
    fn from(button_type: &BasicButtons) -> ButtonColors {
        match button_type {
            BasicButtons::SmallButton => ButtonColors {
                normal: Color::srgba(0.5, 0.5, 0.8, 0.8),
                hovered: Color::srgba(0.5, 0.5, 1., 0.8),
                ..default()
            },
            BasicButtons::MenuButton => ButtonColors {
                normal: Color::srgb(0.3, 0.67, 0.67),
                hovered: Color::srgb(0.3, 0.85, 0.85),
                ..default()
            }
        }
    }
}

// Create the basic button from the enum, can modify after creation
impl From<&BasicButtons> for ButtonBundle {
    fn from(button_type: &BasicButtons) -> ButtonBundle {
        let button_color: ButtonColors = button_type.into();
        match button_type {
            BasicButtons::SmallButton => ButtonBundle {
                style: Style {
                    width: Val::Px(170.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                background_color: button_color.normal.into(),
                border_color: button_color.border.unwrap_or(Color::NONE).into(),
                ..default()
            },
            BasicButtons::MenuButton => ButtonBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(75.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: button_color.normal.into(),
                border_color: button_color.border.unwrap_or(Color::NONE).into(),
                ..default()
            },
        }
    }
}

/// ChangeGameState - Changes the game state when pressed (up)
/// CallFn - Calls a specified FnOnce
#[derive(Component)]
pub enum ButtonAction {
    ChangeGameState(GameState),
    OpenWebLink(&'static str),
}

fn click_menu_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &ButtonColors,
        &ButtonAction,
    ),
    (Changed<Interaction>, With<Button>)>
) {
    use Interaction::*;
    use ButtonAction::*;
    for (interaction, mut bkcolor, mut bdcolor, button_colors, action)
        in &mut interaction_query {
        match *interaction {
            Pressed => {
                match action {
                    ChangeGameState(state) => next_state.set(state.clone()),
                    OpenWebLink(url) => {
                        if let Err(error) = webbrowser::open(url) {
                            warn!("Failed to open link {error:?}")
                        }
                    },
                }
            },
            Hovered => {
                *bkcolor = button_colors.hovered.into();
                *bdcolor = button_colors.border_hovered.unwrap_or(Color::NONE).into();
            },
            None => {
                *bkcolor = button_colors.normal.into();
                *bdcolor = button_colors.border.unwrap_or(Color::NONE).into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

