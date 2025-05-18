use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                spawn_ball,
                spawn_paddle,
                spawn_gutter,
                spawn_camera,
                spawn_scoreboard,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                handle_player_input,
                move_ai,
                detect_scoring,
                reset_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                move_paddle.after(handle_player_input),
                project_position.after(move_ball),
                handle_collisions.after(move_ball),
                update_scoreboard,
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 1.)));
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct Shape(Vec2);

#[derive(Component)]
#[require(
    Position,
    Velocity = Velocity(Vec2::new(0.87, 1.)),
    Shape = Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
)]
struct Ball;

#[derive(Component)]
#[require(Position, Shape = Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)), Velocity)]
struct Paddle;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

#[derive(Resource, Default)]
struct Score {
    player: u32,
    ai: u32,
}

enum Scorer {
    Player,
    Ai,
}

#[derive(Event)]
struct Scored(Scorer);

const GUTTER_HEIGHT: f32 = 20.;
const BALL_SIZE: f32 = 5.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;
const PADDLE_SPEED: f32 = 5.0;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let shape = Circle::new(BALL_SIZE);
    let color = Color::srgb(1.0, 0., 0.);

    let mesh = meshes.add(shape);
    let material = materials.add(color);

    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddle...");

    if let Ok(window) = window.single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
        let mesh = meshes.add(shape);

        let player_color = materials.add(Color::srgb(0.0, 1., 0.));
        let ai_color = materials.add(Color::srgb(0.0, 0., 1.));

        commands.spawn((
            Player,
            Paddle,
            Position(Vec2::new(right_paddle_x, 0.)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(player_color),
        ));

        commands.spawn((
            Ai,
            Paddle,
            Position(Vec2::new(left_paddle_x, 0.)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(ai_color),
        ));
    }
}

fn project_position(mut positions: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positions {
        transform.translation = position.0.extend(0.);
    }
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else {
        if offset.y > 0. {
            Collision::Top
        } else {
            Collision::Bottom
        }
    };

    Some(side)
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                match collision {
                    Collision::Left => ball_velocity.0.x *= -1.,
                    Collision::Right => ball_velocity.0.x *= -1.,
                    Collision::Top => ball_velocity.0.y *= -1.,
                    Collision::Bottom => ball_velocity.0.y *= -1.,
                }
            }
        }
    }
}

fn spawn_gutter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddle...");

    if let Ok(window) = window.single() {
        let window_height = window.resolution.height();
        let window_width = window.resolution.width();

        let shape = Rectangle::new(window_width, GUTTER_HEIGHT);
        let mesh = meshes.add(shape);

        let color = materials.add(Color::srgb(1., 1., 1.));

        let top = window_height / 2. - GUTTER_HEIGHT / 2.;
        let bottom = -window_height / 2. + GUTTER_HEIGHT / 2.;

        commands.spawn((
            Gutter,
            Shape(shape.size()),
            Position(Vec2::new(0., top)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(color.clone()),
        ));

        commands.spawn((
            Gutter,
            Shape(shape.size()),
            Position(Vec2::new(0., bottom)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(color.clone()),
        ));
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = player.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1. * PADDLE_SPEED;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1. * PADDLE_SPEED;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn move_paddle(
    mut paddle: Query<(&Velocity, &mut Position), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT / 2. - PADDLE_HEIGHT / 2.;

        for (velocity, mut position) in &mut paddle {
            let new_position = position.0.y + velocity.0.y;
            if new_position.abs() < max_y {
                position.0.y = new_position;
            }
        }
    }
}

fn detect_scoring(
    ball: Query<&Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.single() {
            if ball.0.x > window_width / 2. {
                events.write(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.write(Scored(Scorer::Player));
            }
        }
    }
}

fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.single_mut() {
            position.0 = Vec2::new(0., 0.);
            match event.0 {
                Scorer::Player => velocity.0 = Vec2::new(-1., 1.),
                Scorer::Ai => velocity.0 = Vec2::new(1., 1.),
            }
        }
    }
}
fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Player => score.player += 1,
            Scorer::Ai => score.ai += 1,
        }
    }

    println!("Score: {} - {}", score.player, score.ai);
}

#[derive(Component)]
struct PlayerScore;

#[derive(Component)]
struct AiScore;

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        PlayerScore,
        Text::new("0"),
        TextFont {
            font_size: 72.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(15.),
            ..default()
        },
    ));

    commands.spawn((
        AiScore,
        Text::new("0"),
        TextFont {
            font_size: 72.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(15.),
            ..default()
        },
    ));
}

fn update_scoreboard(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut text) = player_score.single_mut() {
            text.0 = score.player.to_string();
        }

        if let Ok(mut text) = ai_score.single_mut() {
            text.0 = score.ai.to_string();
        }
    }
}

fn move_ai(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = ai.single_mut() {
        if let Ok(ball_position) = ball.single() {
            let a_to_b = ball_position.0 - position.0;
            velocity.0.y = a_to_b.y.signum();
        }
    }
}
