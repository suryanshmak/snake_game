use std::collections::LinkedList;

use ggez::{ graphics, input::keyboard::KeyCode, Context};
use oorandom::Rand32;

const FPS: u32 = 8;

// define sizes
const BOARD: (i16, i16) = (40, 40);
const BLOCK: (u32, u32) = (32, 32);

const SCREEN: (f32, f32) = (
    BOARD.0 as f32 * BLOCK.0 as f32,
    BOARD.1 as f32 * BLOCK.1 as f32,
);

#[derive(Copy, Clone, Debug, PartialEq)]
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
            Direction::Up => Position::new(pos.x, (pos.y - 1).rem_euclid(BOARD.1)),
            Direction::Down => Position::new(pos.x, (pos.y + 1).rem_euclid(BOARD.1)),
            Direction::Left => Position::new((pos.x - 1).rem_euclid(BOARD.0), pos.y),
            Direction::Right => Position::new((pos.x + 1).rem_euclid(BOARD.0), pos.y),
        }
    }
}

impl From<(i16, i16)> for Position {
    fn from(pos: (i16, i16)) -> Self {
        Position {x: pos.0, y: 1}
    }
}

impl From<Position> for graphics::Rect {
    fn from(pos: Position) -> Self {
        graphics::Rect::new_i32(
           pos.x as i32 * BLOCK.0 as i32, 
           pos.y as i32 * BLOCK.1 as i32,
           BLOCK.0 as i32,
           BLOCK.1 as i32,
        )
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

struct Food(Position);

impl Food {
    fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest_rect(self.0.into()).color([0.0, 0.0, 1.0, 1.0]));
    }
}

struct Snake {
    head: Segment,
    body: LinkedList<Segment>,
    dir: Direction,
    last_dir: Direction,
    next_dir: Option<Direction>,
    touched: Option<Touched>,
}

#[derive(Clone, Copy, Debug)]
enum Touched {
    Body,
    Food
}

impl Snake {
    fn new(pos: Position) -> Self {
        let mut body = LinkedList::new();

        body.push_back(Segment((pos.x-1, pos.y).into()));

        Self {
            head: Segment(pos),
            body,
            dir: Direction::Right,
            last_dir: Direction::Right,
            next_dir: None,
            touched: None,
        }
    }

    fn ate_food(&self, food: &Food) -> bool {
        self.head.0 == food.0
    }

    fn eats_body(&self) -> bool {
        self.body.iter().any(|segment| segment.0 == self.head.0)
    }

    fn update(&mut self, food: &Food) {
        if self.last_dir == self.dir && self.next_dir.is_some() {
            self .dir = self.next_dir.unwrap();
            self.next_dir = None;
        }

        let new_head_pos = Position::new_from_move(self.head.0, self.dir);

        let new_head = Segment(new_head_pos);

        self.body.push_front(new_head);

        self.head = new_head;

        if self.eats_body() {
            self.touched = Some(Touched::Body);
        } else if self.ate_food(food) {
            self.touched = Some(Touched::Food);
        } else {
            self.touched = None;
        }

        if self.touched.is_none() {
            self.body.pop_back();
        }

        self.last_dir = self.dir;
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest_rect(self.head.0.into()).color([1.0, 0.5, 0.0, 1.0]));
        for segment in self.body.iter() {
            canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest_rect(segment.0.into()).color([1.0, 0.5, 0.0, 1.0]));
        }
    }
}

struct GameState {
    over: bool,
    rng: Rand32,
    snake: Snake,
    food: Food,
}

impl GameState {
    fn new() -> Self {
        let snake = Snake::new((BOARD.0 / 4, BOARD.1 / 2).into());

        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Failed to get random seed");

        let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        
        let food = Food(Position::random(&mut rng, BOARD.0, BOARD.1));

        Self { over: false, rng, snake, food }
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        while ctx.time.check_update_time(FPS) {
            if !self.over {
                self.snake.update(&self.food);

                if let Some(touched) = self.snake.touched {
                    match touched {
                        Touched::Body => self.over = true,
                        Touched::Food => {
                            self.food = Food(Position::random(&mut self.rng, BOARD.0, BOARD.1));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::CanvasLoadOp::Clear([0.0, 1.0, 0.0, 1.0].into()));

        self.food.draw(&mut canvas);
        self.snake.draw(&mut canvas);

        canvas.finish(ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool
        ) -> Result<(), ggez::GameError> {
            if let Some(dir) = input.keycode.and_then(Direction::from_key) {
                if self.snake.dir != self.snake.last_dir && self.snake.dir != dir.inverse() {
                    self.snake.next_dir = Some(dir);
                } else {
                    self.snake.dir = dir; 
                }
            }
        Ok(())
    }
}

fn main() {
    let (ctx, event_loop) = ggez::ContextBuilder::new("snake", "suryanshmak")
    .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN.0, SCREEN.1))
    .build()
    .expect("Failed to initialize ggez");

    let state = GameState::new();
    ggez::event::run(ctx, event_loop, state);
}