use bevy::{
    prelude::{App, AssetMode, AssetPlugin, ClearColor, Color, ImagePlugin, PluginGroup},
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_stats::StatPlugin;
use bevy_twin_stick::TwinStickPlugin;

pub mod gamestate;

pub const BACKGROUND_COLOR: Color = Color::hsl(200., 0.9, 0.04);

fn main() {
    let mut app = App::new();

    let asset_plugin = AssetPlugin {
        file_path: String::new(),
        processed_file_path: String::new(),
        watch_for_changes_override: Some(true),
        mode: AssetMode::Processed,
    };

    //default bevy plugins
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(asset_plugin)
                .build(),
        );

    //external crates
    app.add_plugins((
        ScreenDiagnosticsPlugin::default(),
        ScreenFrameDiagnosticsPlugin,
    ));
    app.add_plugins((TwinStickPlugin, StatPlugin));

    //debug-only external crates
    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
        // app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}
