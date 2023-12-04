use bevy::{
    app::AppExit,
    ecs::{event::EventWriter, schedule::common_conditions::in_state, system::ResMut},
    prelude::{IntoSystemConfigs, NextState, OnEnter, Plugin, States, Update},
    reflect::Reflect,
};
use bevy_egui::{egui, EguiContexts};

use crate::level::initial_world_setup;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<AppState>();
        app.add_systems(Update, main_menu_gui.run_if(in_state(AppState::MainMenu)));
        app.add_systems(OnEnter(AppState::Quitting), exit);
        app.add_systems(OnEnter(AppState::InGame), initial_world_setup);
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AppState {
    #[default]
    LoadingAssets,
    MainMenu,
    InGame,
    Quitting,
}

pub fn main_menu_gui(mut root: EguiContexts, mut state: ResMut<NextState<AppState>>) {
    egui::CentralPanel::default().show(root.ctx_mut(), |ui| {
        ui.allocate_space(egui::Vec2::new(1.0, 200.0));

        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label("Too Many Cooks");
                if ui.button("Play Game").clicked() {
                    state.set(AppState::InGame);
                }
                if ui.button("Options").clicked() {
                    todo!();
                }
                if ui.button("Quit").clicked() {
                    state.set(AppState::Quitting);
                }
            })
        });
    });
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
