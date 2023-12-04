use appstate::AppStatePlugin;
use assets::AssetsPlugin;
use bevy::{
    prelude::{App, ClearColor, Color, ImagePlugin, PluginGroup},
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_stats::StatPlugin;
use bevy_twin_stick::TwinStickPlugin;

pub mod appstate;
pub mod assets;
pub mod characters;
pub mod level;

pub const BACKGROUND_COLOR: Color = Color::hsl(200., 0.9, 0.04);

fn main() {
    let mut app = App::new();

    //default bevy plugins
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build());

    // internal top-level crates
    app.add_plugins((AppStatePlugin, AssetsPlugin));

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
