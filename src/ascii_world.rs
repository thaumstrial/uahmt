use bevy::prelude::*;

#[derive(Component)]
pub struct AsciiTile {
    pub pos: UVec3,
}

#[derive(Resource)]
pub struct WorldSettings {
    pub size: UVec3
}
impl FromWorld for WorldSettings {
    fn from_world(world: &mut World) -> Self {
        Self {
            size: UVec3::new(64, 64, 64)
        }
    }
}

#[derive(Event)]
pub struct AsciiAddEvent {
    pub entity: Entity,
    pub pos: UVec3
}
#[derive(Event)]
pub struct AsciiRemoveEvent(Entity);
#[derive(Event)]
pub struct AsciiMoveEvent {
    pub entity: Entity,
    pub old_pos: UVec3,
    pub new_pos: UVec3
}

fn startup(
    mut event: EventWriter<AsciiAddEvent>,
    mut commands: Commands
) {
    let pos = UVec3::default();
    let entity = commands.spawn(AsciiTile {pos}).id();
    event.send(AsciiAddEvent {
        entity,
        pos
    });
}
pub struct AsciiWorldPlugin;
impl Plugin for AsciiWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldSettings>()
            .add_systems(Startup, startup)
            .add_event::<AsciiAddEvent>()
            .add_event::<AsciiRemoveEvent>()
            .add_event::<AsciiMoveEvent>();
    }
}