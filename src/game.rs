use bevy::prelude::*;

// Define the system set for all behaviours within the game
#[derive(SystemSet, Clone, Debug, Eq, PartialEq, Hash)]
struct GameplaySet;

#[derive(SystemSet, Clone, Debug, Eq, PartialEq, Hash)]
struct InputSet;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(ActionPlugin);
            // .add_systems(Update, propagate_tentacles)
    }
}

//// All the entities and behaviours within each game level
#[derive(Component, Clone)]
pub enum CellOwner {
    Player,
    Enemy(u8),
    None,
}

pub trait Extendable {
    fn extend_tentacle(&self, other: Entity);
    fn cut_tentacle(&self, cut_place: f32);
}


// Health
#[derive(Component)]
pub struct Health {
    health: usize
}

// Collection of the tentacles associated with the cell
#[derive(Component, Clone)]
pub struct Tentacles {
    max_tentacles: usize,
    tentacle_targets: Vec<Entity>
}

impl Extendable for Tentacles {
    fn extend_tentacle(&self, other: Entity) {

    }
}

#[derive(Bundle)]
pub struct BioCellBundle {
    health: Health,
    cell: CellOwner,
    tentacles: Tentacles,
}

