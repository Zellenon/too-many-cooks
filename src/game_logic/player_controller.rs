use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::core::Name;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                add_test_player,
            ))
            .add_systems(Update, (
                point_player_at_mouse,
                override_angular_velocity,
                player_acceleration,
            ));
    }
}

// TODO: remove when done testing player controller
fn add_test_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        PlayerShipBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("assets/sprites/ship.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            // size of ship collider
            collider: Collider::ball(40.0),
            ..default()
        },
        Name::new("PlayerEntity".to_string()),
    ));
}

/// marker to specify that an entity is a player controller
#[derive(Component)]
pub struct Player;

/// marker to specify that an entity is a ship
#[derive(Component)]
pub struct Ship;

/// Ship bundle
#[derive(Bundle)]
pub struct PlayerShipBundle {
    pub player: Player,
    pub ship: Ship,
    pub sprite_bundle: SpriteBundle,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub gravity: GravityScale,
}

impl Default for PlayerShipBundle {
    fn default() -> Self {
        Self {
            player: Player,
            ship: Ship,
            sprite_bundle: SpriteBundle::default(),
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(1.0),
            external_force: ExternalForce::default(),
            gravity: GravityScale(0.0),
        }
    }
}

/// Systems ///

/// Stops rapier from controlling the angular velocity of the player.
/// Gives full control to the player controller.
fn override_angular_velocity(
    mut player_info: Query<&mut Velocity, (With<Player>, With<Ship>)>,
) {
    for mut velocity in player_info.iter_mut() {
        velocity.angvel = 0.0;
    }
}

/// Player ship acceleration
fn player_acceleration(
    mut player_info: Query<(&mut ExternalForce, &Transform), (With<Player>, With<Ship>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut external_force, transform) in player_info.iter_mut() {
        let (y, x) = transform.rotation.to_euler(EulerRot::ZYX).0.sin_cos();
        let direction_vec = Vec2::new(x, y) * 100.0; // TODO: replace hardcoded value
    
        if keyboard_input.pressed(KeyCode::W) {
            external_force.force = direction_vec;
        } else {
            external_force.force = Vec2::new(0.0, 0.0);
        }
    }
}

/// Self explainatory
fn point_player_at_mouse (
    mut player_info: Query<&mut Transform, With<Player>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    for mut transform in player_info.iter_mut() {
        // only works if the cursor is inside the window
        // assuming that there is exactly one window
        if let Some(mouse_position) = windows.single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()) {

            // turn transform into Vec2
            let player_position = Vec2::new(transform.translation.x, transform.translation.y);
            let relative_mouse_position = mouse_position - player_position;
            
            // get angle between player and mouse
            let angle = relative_mouse_position.y.atan2(relative_mouse_position.x);

            // rotate player to face mouse
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}