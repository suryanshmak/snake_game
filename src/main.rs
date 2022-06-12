use bevy::prelude::*;
use oorandom::Rand32;

const FPS: u32 = 8;

// define sizes
const BOARD: (u32, u32) = (40, 40);
const BLOCK: (u32, u32) = (32, 32);

const SCREEN: (f32, f32) = (
    BOARD.0 as f32 * BLOCK.0 as f32,
    BOARD.1 as f32 * BLOCK.1 as f32,
);

#[derive(Component, Copy, Clone, Debug)]
struct Position { x: i16, y: i16 }

impl Position {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn random(rng: &mut Rand32, max_x: i16, max_y: i16) -> Self {
        (
           rng.rand_range(0..(max_x as u32)) as i16,
           rng.rand_range(0..(max_y as u32)) as i16
        ).into()
    }

    pub fn new_from_move(pos: Position, dir: Direction) -> Self {
        match dir {
            Direction::Up => Position::new(pos.x, (pos.y - 1).rem_euclid(BOARD.1 as i16)),
            Direction::Down => Position::new(pos.x, (pos.y + 1).rem_euclid(BOARD.1 as i16)),
            Direction::Left => Position::new((pos.x - 1).rem_euclid(BOARD.0 as i16), pos.y),
            Direction::Right => Position::new((pos.x + 1).rem_euclid(BOARD.0 as i16), pos.y),
        }
    }
}

impl From<(i16, i16)> for Position {
    fn from(pos: (i16, i16)) -> Self {
        Position {x: pos.0, y: pos.1}
    }
}

impl From<&Position> for bevy::sprite::Rect {
    fn from(pos: &Position) -> Self {
        bevy::sprite::Rect {
           min: Vec2::new(pos.x as f32 * BLOCK.0 as f32, pos.y as f32 * BLOCK.1 as f32),
           max: Vec2::new(BLOCK.0 as f32, BLOCK.1 as f32),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn from_key(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment(Position);

impl Segment {
    pub fn new(pos: Position) -> Self {
        Self(pos)
    }
}

#[derive(Component)]
struct Food;

fn main() {
    App::new().insert_resource(WindowDescriptor {
        title: "Snake".to_string(),
        width: SCREEN.0,
        height: SCREEN.1,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(draw_food)
    .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

}

fn draw_food(mut commands: Commands, query: Query<(&Position, With<Food>)>) {
    // if let (pos, _) = query.single_mut() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        });
    // }
}