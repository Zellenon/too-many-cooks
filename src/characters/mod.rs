use bevy::prelude::{Entity, Handle, Image, Name, Vec2};
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::{tree::ComponentTree, CT};
use bevy_twin_stick::actors::{ActorBundle, Tracking};
use bevy_twin_stick::bevy_mod_transform2d::transform2d::Transform2d;
use bevy_twin_stick::stats::Speed;

use crate::assets::add_texture;

pub mod mooks;
pub mod player;

pub fn world_actor(name: &'static str, speed: f32, tex: &Handle<Image>) -> ComponentTree {
    CT!(Name::new(name), ActorBundle::default(), Speed(speed)) + add_texture(tex)
}

/// Returns a CT that can be used to easily shift the starting position of an entity by overwriting
/// the default transform provided by a previous CT.
pub fn shift_pos(pos: impl Into<Vec2>) -> ComponentTree {
    let new_pos = pos.into();
    CT!(Transform2d::from_translation(new_pos))
}

/// Returns a CT that can be used to easily shift the tracking state of an entity by overwriting
/// the default tracking provided by a previous CT.
pub fn shift_tracking(tracking: Option<Entity>) -> ComponentTree {
    CT!(Tracking(tracking))
}
