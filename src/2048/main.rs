use crate::ui::GameUiPlugin;
use bevy::DefaultPlugins;
use bevy::color::palettes::tailwind::SLATE_950;
use bevy::prelude::*;

mod ui;

#[derive(Default, Resource)]
struct Game {
    score: u32,
    score_best: u32,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
enum RunState {
    #[default]
    Startup,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Srgba::hex("#1f2638").unwrap().into()))
        .insert_resource(Board::new(4))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "2048".into(),
                    ..default()
                }),
                ..default()
            }),
            GameUiPlugin,
        ))
        .init_resource::<Game>()
        .run();
}

#[derive(Resource)]
struct Board {
    size: u32,
}


impl Board {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}

//
// #[derive(Component)]
// struct Brick;
// const CANVAS_SIZE: Vec2 = Vec2::new(100., 200.);
//
// fn new_game(commands: &mut Commands) {
//     let mut meshes = vec![];
//     let mut materials = vec![];
//
//     let brick_size = Vec2::new(80., 40.);
//     let num_bricks_per_row = 6;
//     let rows = 4;
//     for row in 0..rows {
//         for i in 0..num_bricks_per_row {
//             commands.spawn((
//                 Brick,
//                 Mesh2d(meshes.add(Rectangle::new(brick_size.x, brick_size.y))),
//                 MeshMaterial2d(materials.add(Color::from(SLATE_950).with_alpha(0.2))),
//                 Transform::from_xyz(
//                     brick_size.x * i as f32 - brick_size.x * num_bricks_per_row as f32 / 2.
//                         + brick_size.x / 2.,
//                     CANVAS_SIZE.y * (3. / 8.) - brick_size.y * row as f32,
//                     0.0,
//                 ),
//                 StateScoped(AppState::Playing)
//             ));
//         }
//     }
// }
