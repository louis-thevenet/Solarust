use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct CustomPerfUiPlugin;
impl Plugin for CustomPerfUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PerfUiPlugin,
            FrameTimeDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(Startup, perf_ui);
    }
}
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
