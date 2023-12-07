use bevy::render::render_resource::AsBindGroup;
use bevy::{prelude::*, window::PresentMode};
use bevy::window::{WindowResolution, WindowMode};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
//https://github.com/laundmo/bevy_screen_diagnostics
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use too_many_cooks::game_logic;
use bevy_rapier2d::prelude::*;
use too_many_cooks::CorePlugin;
use crate::game_logic::raycast::RayCastPlugin;

pub const BACKGROUND_COLOR: Color = Color::hsl(200.,0.9, 0.04);

fn main() {
    let mut app = App::new();

    let asset_plugin = AssetPlugin {
        file_path: String::new(),
        processed_file_path: String::new(),
        watch_for_changes_override: Some(true),
        mode: AssetMode::Processed,
    };

    let window_plugin: WindowPlugin = WindowPlugin {
        primary_window: Some(Window {
            title: "too_many_cooks".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::AutoNoVsync,
            fit_canvas_to_parent: true,
            resolution: WindowResolution::new(1200., 750.),
            resizable: false,
            ..default()
        }),
        ..default()
    };

    //default bevy plugins
    app
        .insert_resource(ClearColor(
            BACKGROUND_COLOR,
        ))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(asset_plugin)
                .set(window_plugin)
                .build(),
        );
    
    // core plugin
    app.add_plugins(CorePlugin);
    
    app.add_plugins(RayCastPlugin);
    
    //external crates
    app
        .add_plugins((
            WorldInspectorPlugin::new(),
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
    ));
    
    // game logic
    app
        .add_plugins((
            game_logic::player_controller::PlayerControllerPlugin,
        ));

    app.run();
}
