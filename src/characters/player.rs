use bevy_composable::tree::EntityCommandSet;

use bevy_composable::{tree::ComponentTree, CT};
use bevy_twin_stick::{ai::keyboard::KeyboardAI, player::Player};

use crate::assets::ImageAssets;

use super::world_actor;

pub fn player_character(sprites: &ImageAssets) -> ComponentTree {
    world_actor("Player", 800., &sprites.the_transient) + CT!(Player, KeyboardAI)
}
