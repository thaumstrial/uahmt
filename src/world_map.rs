use bevy::prelude::*;

fn startup(
    mut commands: Commands
) {

}
pub(crate) struct WorldMapPlugin;
impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}