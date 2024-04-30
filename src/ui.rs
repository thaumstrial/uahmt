use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_fast_tilemap::{Map, MapBundleManaged};
use crate::ascii_render::{AsciiAtlas, UserData};
use crate::MainState;
use std::convert::TryFrom;
use std::time::Duration;
use rand::Rng;

#[derive(Component)]
struct UpdateTime(Timer);
#[derive(Component)]
struct BannerTiles(Vec<UVec2>);
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum MainMenuState {
    #[default]
    Continue,
    Connect,
    New,
    Load,
    Setting,
}

fn banner_effect(
    mut materials: ResMut<Assets<Map<UserData>>>,
    maps: Query<&Handle<Map<UserData>>>,
    mut q: Query<(&mut UpdateTime, &mut BannerTiles)>,
    time: Res<Time>,
) {
    if let Ok((mut timer, mut tiles)) = q.get_single_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let rng_tile = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;':\",./<>?`~";
            let rng_color = vec![
                Color::RED,
                Color::GREEN,
                Color::BLUE,
                Color::YELLOW,
                Color::PURPLE,
                Color::ORANGE,
            ];
            let mut rng = rand::thread_rng();
            let map = materials.get_mut(maps.get_single().unwrap()).unwrap();
            let mut m = map.indexer_mut();
            m.set_uvec(tiles.0[rng.gen_range(0..tiles.0.len())], rng_tile.chars().nth(rng.gen_range(0..rng_tile.len())).unwrap() as u32, rng_color[rng.gen_range(0..rng_color.len())], Color::BLACK);
        }
    }
}
fn draw_main_menu(
    ascii_atlas: Res<AsciiAtlas>,
    mut materials: ResMut<Assets<Map<crate::ascii_render::UserData>>>,
    mut commands: Commands
) {
    let banner = [
        " ,ggg,         gg                                                  ".chars().collect::<Vec<_>>(),
        "dP\"\"Y8a        88              ,dPYb,                         I8   ".chars().collect::<Vec<_>>(),
        "Yb, `88        88              IP'`Yb                         I8   ".chars().collect::<Vec<_>>(),
        "`\"\"  88        88              I8  8I                      88888888".chars().collect::<Vec<_>>(),
        "     88        88              I8  8'                         I8   ".chars().collect::<Vec<_>>(),
        "     88        88    ,gggg,gg  I8 dPgg,    ,ggg,,ggg,,ggg,    I8   ".chars().collect::<Vec<_>>(),
        "     88        88   dP\"  \"Y8I  I8dP\" \"8I  ,8\" \"8P\" \"8P\" \"8,   I8   ".chars().collect::<Vec<_>>(),
        "     88        88  i8'    ,8I  I8P    I8  I8   8I   8I   8I  ,I8,  ".chars().collect::<Vec<_>>(),
        "     Y8b,____,d88,,d8,   ,d8b,,d8     I8,,dP   8I   8I   Yb,,d88b, ".chars().collect::<Vec<_>>(),
        "      \"Y888888P\"Y8P\"Y8888P\"`Y888P     `Y88P'   8I   8I   `Y88P\"\"Y8 ".chars().collect::<Vec<_>>(),
        "                                                                   ".chars().collect::<Vec<_>>(),
        "     Continue      Connect     New        Load       Setting       ".chars().collect::<Vec<_>>(),
        "                                                                   ".chars().collect::<Vec<_>>(),
    ];

    let mut banner_tiles = vec![];

    let map = Map::<crate::ascii_render::UserData>::builder(
        UVec2::new(67, 13),
        ascii_atlas.0.clone(),
        vec2(16., 16.),
    )
        .with_user_data(crate::ascii_render::UserData { alpha: 1.})
        .build_and_initialize(
            |m| {
                for y in 0..m.size().y {
                    for x in 0..m.size().x {
                        let tile = banner[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] as u32;
                        m.set(x, y, tile, Color::WHITE, Color::NONE);
                        if y < 10 && tile != 32 {
                            banner_tiles.push(UVec2::new(x, y));
                        }
                    }
                }
            }
        );
    commands.spawn(MapBundleManaged::<crate::ascii_render::UserData> {
        material: materials.add(map),
        transform: Transform::default().with_translation(vec3(0., 0., 0.)),
        ..default()
    })
        .insert(UpdateTime(Timer::new(Duration::from_millis(500), TimerMode::Repeating)))
        .insert(BannerTiles(banner_tiles));
}

pub(crate) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MainMenuState>()
            .add_systems(OnEnter(MainState::MainMenu), draw_main_menu)
            .add_systems(Update, banner_effect.run_if(in_state(MainState::MainMenu)));
    }
}