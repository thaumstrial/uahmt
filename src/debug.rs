use bevy::asset::ron::de::Position;
use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use bevy::diagnostic::*;
use bevy::ecs::system::lifetimeless::SQuery;
use bevy::ecs::system::SystemParam;
use iyes_perf_ui::utils::next_sort_key;
use crate::ascii_render::ViewLayer;
use crate::ascii_world::AsciiTile;
use crate::player::{PlayerMarker, PlayerPlugin};

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

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryPlayerPosition {
    pub label: String,
    pub separator: &'static str,
    pub position: Option<UVec3>,
    pub width: u8,
    pub sort_key: i32,
}
impl Default for PerfUiEntryPlayerPosition {
    fn default() -> Self {
        Self {
            label: String::new(),
            separator: ", ",
            position: None,
            width: 20,
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryPlayerPosition {
    type SystemParam = (SQuery<&'static AsciiTile, With<PlayerMarker>>);
    type Value = UVec3;
    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Player Position"
        } else {
            &self.label
        }
    }
    fn update_value(&self, tile: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        if let Ok(tile) = tile.get_single() {
            Some(tile.pos)
        } else {
            None
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format!(
            "X: {}{}Y: {}{}Z: {}",
            value.x, self.separator, value.y, self.separator, value.z
        )
    }
}
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryViewLayer {
    pub label: String,
    pub position: Option<UVec3>,
    pub width: u8,
    pub sort_key: i32,
}
impl Default for crate::debug::PerfUiEntryViewLayer {
    fn default() -> Self {
        Self {
            label: String::new(),
            position: None,
            width: 8,
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryViewLayer {
    type SystemParam = (SQuery<&'static ViewLayer>);
    type Value = u32;
    fn label(&self) -> &str {
        if self.label.is_empty() {
            "View Layer"
        } else {
            &self.label
        }
    }
    fn update_value(&self, tile: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        if let Ok(view) = tile.get_single() {
            Some(view.0)
        } else {
            None
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format!(
            "{}",
            value
        )
    }
}

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
            PerfUiEntryPlayerPosition::default(),
            PerfUiEntryViewLayer::default()
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
            .add_perf_ui_entry_type::<PerfUiEntryPlayerPosition>()
            .add_perf_ui_entry_type::<PerfUiEntryViewLayer>()
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