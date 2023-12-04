use bevy::prelude::{Commands, Res};
use bevy_composable::app_impl::ComplexSpawnable;

use crate::{assets::ImageAssets, characters::player::player_character};

pub fn initial_world_setup(mut commands: Commands, sprites: Res<ImageAssets>) {
    commands.spawn_complex(player_character(&*sprites));
}
