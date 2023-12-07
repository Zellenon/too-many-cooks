use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::core::Name;
use crate::game_logic::raycast::{RayCast, RayCastEvent};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                add_test_player,
            ))
            .add_systems(PreUpdate, (
                zero_out_external_forces,
            ))
            .add_systems(Update, (
                point_player_at_mouse,
                limit_ship_speed,
                zero_out_angular_velocity,
                player_acceleration,
                // raycast_push_object,
                loop_on_edge,
                raycast_push_ship,
            ));
    }
}

fn add_test_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        PlayerShipBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("assets/sprites/ship_transparent.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            collider: Collider::ball(40.0),
            ship: Ship {
                engine_force: 200.0,
                max_vel: 300.0,
            },
            ..default()
        },
        Name::new("PlayerEntity".to_string()),
    )).with_children(|parent| {
        let num_raycasts = 5;
        let spread_angle: f32 = 5.0f32.to_radians();
        for i in 0..num_raycasts {
            parent.spawn((
                RayCast {
                    max_distance: 500.0,
                    angle: std::f32::consts::PI + spread_angle * (i as f32 - (num_raycasts as f32 - 1.0) / 2.0),
                },
                TransformBundle {
                    local: Transform::from_xyz(-40.001, 0.0, 0.0),
                    ..default()
                },
            ));
        }
    });
    
    // test obstacle
    // commands.spawn((
    //     Transform::from_xyz(200.0, 0.0, 0.0),
    //     RigidBody::Dynamic,
    //     Collider::cuboid(100.0, 400.0),
    //     GravityScale(0.0),
    //     ExternalForce::default(),
    // ));

    // create an bunch of obstacles in one big arc
    for i in 0..20 {
        let angle = i as f32 * 2.0 * std::f32::consts::PI / 20.0;
        let x = angle.cos() * 400.0;
        let y = angle.sin() * 400.0;
        commands.spawn((
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(10.0, 400.0),
            GravityScale(0.0),
            ExternalForce::default(),
        ));
    }
    // commands.spawn((
    //     TransformBundle::from(Transform::from_xyz(0.0, -400.0, 0.0)),
    //     RigidBody::Fixed,
    //     Collider::cuboid(1000.0, 10.0),
    //     GravityScale(0.0),
    // ));
}

/// marker to specify that an entity is a player controller
#[derive(Component)]
pub struct Player;

/// marker to specify that an entity is a ship
#[derive(Component)]
pub struct Ship {
    engine_force: f32,
    max_vel: f32,
}

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
            ship: Ship {
                engine_force: 100.0,
                max_vel: 100.0,
            },
            sprite_bundle: SpriteBundle::default(),
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(1.0),
            external_force: ExternalForce::default(),
            gravity: GravityScale(0.0),
        }
    }
}

/// Systems ///

fn zero_out_external_forces(
    mut external_forces: Query<&mut ExternalForce>,
) {
    for mut force in external_forces.iter_mut() {
        force.force = Vec2::new(0.0, 0.0);
        force.torque = 0.0;
    }
}

fn raycast_push_object (
    mut raycast_events: EventReader<RayCastEvent>,
    mut external_forces: Query<(&mut ExternalForce, &Transform)>,
) {
    // push every object hit by the raycasts according to where they were hit
    for raycast_event in raycast_events.read() {
        if let Ok((mut force, transform)) = external_forces.get_mut(raycast_event.collision_entity) {
            // set force and torque according to intersection point, and direction the ray originated from
            let radius_vec = raycast_event.intersection_point - transform.translation.truncate();
            let torque = -radius_vec.length() * raycast_event.direction.length() * (radius_vec.angle_between(-raycast_event.direction)).sin();
            let force_vec = -raycast_event.intersection_normal;
            
            force.force += force_vec;
            // hold off on torque for now
            force.torque += torque;
        }
    }
}

fn raycast_push_ship (
    mut raycast_events: EventReader<RayCastEvent>,
    parent: Query<&Parent>,
    mut external_force: Query<&mut ExternalForce>,
) {
    for raycast_event in raycast_events.read() {
        if let Ok(parent) = parent.get(raycast_event.raycast_entity) {
            // compute strength of force based on distance from center of ship
            let radius_vec = raycast_event.intersection_point - raycast_event.origin;
            let force_strength = 1000.0/(radius_vec.length() + 1.0);
            
            if let Ok(mut force) = external_force.get_mut(parent.get()) {
                force.force += -radius_vec.normalize() * force_strength;
            }
        }
    }
}



/// loops all transforms on edge
fn loop_on_edge(
    mut transforms: Query<&mut Transform>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // assuming that there is exactly one window
    let window = windows.single();
    let window_width = window.physical_width() as f32;
    let window_height = window.physical_height() as f32;

    for mut transform in transforms.iter_mut() {
        if transform.translation.x > window_width / 2.0 {
            transform.translation.x = -window_width / 2.0;
        } else if transform.translation.x < -window_width / 2.0 {
            transform.translation.x = window_width / 2.0;
        }
        
        if transform.translation.y > window_height / 2.0 {
            transform.translation.y = -window_height / 2.0;
        } else if transform.translation.y < -window_height / 2.0 {
            transform.translation.y = window_height / 2.0;
        }
    }
}

/// Stops rapier from controlling the angular velocity of the player.
/// Gives full control to the player controller.
fn zero_out_angular_velocity(
    mut player_info: Query<&mut Velocity, (With<Player>, With<Ship>)>,
) {
    for mut velocity in player_info.iter_mut() {
        velocity.angvel = 0.0;
    }
}

/// Player ship acceleration
fn player_acceleration(
    mut player_info: Query<(&mut ExternalForce, &Transform, &Ship), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut external_force, transform, ship) in player_info.iter_mut() {
        let (y, x) = transform.rotation.to_euler(EulerRot::ZYX).0.sin_cos();
        let direction_vec = Vec2::new(x, y) * ship.engine_force;
    
        if keyboard_input.pressed(KeyCode::W) {
            external_force.force += direction_vec;
        } else {
            external_force.force += Vec2::new(0.0, 0.0);
        }
    }
}

/// limit ship speed
fn limit_ship_speed(
    mut player_info: Query<(&mut Velocity, &Ship)>
) {
    for (mut velocity, ship) in player_info.iter_mut() {
        let speed = velocity.linvel.length();
        if speed > ship.max_vel {
            velocity.linvel = velocity.linvel.normalize() * ship.max_vel;
        }
    }
}

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