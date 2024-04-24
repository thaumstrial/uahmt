use bevy::prelude::*;

#[derive(Component)]
pub struct AsciiTile {
    pub pos: IVec3,
}


fn startup(mut commands: Commands) {
    commands.spawn(AsciiTile {pos: IVec3::default() });
}
pub struct AsciiWorldPlugin;
impl Plugin for AsciiWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}