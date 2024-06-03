use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::prelude::*;

/// Plugin that adds the performance UI.
pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PerfUiPlugin,
            WorldInspectorPlugin::new(),
            FrameTimeDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
        ))
            .add_systems(Startup, perf_ui);
    }
}

/// System that spawns the performance UI components.
fn perf_ui(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot { ..default() },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryCursorPosition::default(),
        PerfUiEntryFrameTime::default(),
        PerfUiEntryCpuUsage::default(),
        PerfUiEntryMemUsage::default(),
    ));
}
