use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use bevy::diagnostic::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum DebugState {
    Enabled,
    #[default]
    Disabled
}
#[derive(Event)]
struct EnableDebugEvent;
#[derive(Event)]
struct DisableDebugEvent;


fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<DebugState>>,
    mut next_state: ResMut<NextState<DebugState>>,
    mut enable_debug: EventWriter<EnableDebugEvent>,
    mut disable_debug: EventWriter<DisableDebugEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        match state.get() {
            DebugState::Enabled => {
                disable_debug.send(DisableDebugEvent);
                next_state.set(DebugState::Disabled)
            },
            DebugState::Disabled => {
                enable_debug.send(EnableDebugEvent);
                next_state.set(DebugState::Enabled)
            },
        }
    }
}

fn show_perf_ui(mut commands: Commands, mut enable_debug: EventReader<EnableDebugEvent>,) {
    for ev in enable_debug.read() {
        commands.spawn((
            PerfUiRoot::default(),
            PerfUiEntryFPSWorst::default(),
            PerfUiEntryFPS::default(),
            PerfUiEntryEntityCount::default(),
            PerfUiEntryWindowResolution::default(),
            PerfUiEntryCursorPosition::default(),

        ));
    }
}

fn hide_perf_ui(mut commands: Commands, perf_ui: Query<Entity, With<PerfUiRoot>>, mut disable_debug: EventReader<DisableDebugEvent>,) {
    for ev in disable_debug.read() {
        if let Ok(e) = perf_ui.get_single() {
            commands.entity(e).despawn_recursive();
        }
    }
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PerfUiPlugin)
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_plugins(SystemInformationDiagnosticsPlugin)
            .add_systems(Update, keyboard_input)
            .add_systems(Update, show_perf_ui.before(iyes_perf_ui::PerfUiSet::Setup))
            .add_systems(Update, hide_perf_ui)
            .add_event::<EnableDebugEvent>()
            .add_event::<DisableDebugEvent>()
            .init_state::<DebugState>();
    }
}