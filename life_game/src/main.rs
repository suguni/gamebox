use bevy::prelude::*;
use life_game::Grid;

const CELL_SIZE: f32 = 10.0;

#[derive(Resource)]
struct GameGrid(Grid);

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Component)]
struct CellIndex(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameGrid(Grid::random(80, 50)))
        .insert_resource(GameTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, render))
        .run();
}

fn update(mut game_grid: ResMut<GameGrid>, mut game_timer: ResMut<GameTimer>, time: Res<Time>) {
    if game_timer.0.tick(time.delta()).just_finished() {
        let grid = &mut game_grid.0;
        *grid = grid.next_generation();
    }
}

fn render(game_grid: Res<GameGrid>, mut query: Query<(&CellIndex, &mut Sprite)>) {
    for (CellIndex(index), mut sprite) in query.iter_mut() {
        let color = if game_grid.0.cells[*index] {
            Color::WHITE
        } else {
            Color::BLACK
        };
        sprite.color = color;
    }
}

fn setup(mut commands: Commands, game_grid: Res<GameGrid>) {
    commands.spawn(Camera2d);

    let grid = &game_grid.0;
    let offset_x = -(grid.cols as f32 * CELL_SIZE) / 2.0;
    let offset_y = -(grid.rows as f32 * CELL_SIZE) / 2.0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let x = offset_x + col as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            let y = offset_y + row as f32 * CELL_SIZE + CELL_SIZE / 2.0;

            let index = (row * grid.cols + col) as usize;

            let color = if grid.cells[index] {
                Color::WHITE
            } else {
                Color::BLACK
            };

            let custom_size = Some(Vec2::splat(CELL_SIZE - 1.0));

            commands.spawn((
                CellIndex(index),
                Sprite {
                    color,
                    custom_size,
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
