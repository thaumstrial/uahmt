use bevy::prelude::*;

#[derive(Component)]
pub struct AsciiTile {
    pub pos: UVec3,
}

#[derive(Event)]
pub struct AsciiAddEvent {
    pub entity: Entity,
    pub pos: UVec3
}
#[derive(Event)]
pub struct AsciiRemoveEvent(Entity);
#[derive(Event)]
pub struct AsciiMoveEvent(Entity);

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
            .add_systems(Startup, startup)
            .add_event::<AsciiAddEvent>()
            .add_event::<AsciiRemoveEvent>()
            .add_event::<AsciiMoveEvent>();
    }
}