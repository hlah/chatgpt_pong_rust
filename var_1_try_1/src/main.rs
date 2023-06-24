use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, Mesh},
    input::keyboard::{self, KeyCode},
    nalgebra::Point2,
    timer, Context, GameResult,
};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 400.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 200.0;

struct Paddle {
    x: f32,
    y: f32,
    dy: f32,
}

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Paddle { x, y, dy: 0.0 }
    }

    fn update(&mut self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.dy = -PADDLE_SPEED;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.dy = PADDLE_SPEED;
        } else {
            self.dy = 0.0;
        }

        self.y += self.dy * timer::delta(ctx).as_secs_f32();
        self.y = self.y.clamp(0.0, WINDOW_HEIGHT - PADDLE_HEIGHT);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        Ok(())
    }
}

impl Ball {
    fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Ball { x, y, dx, dy }
    }

    fn update(&mut self, ctx: &mut Context) {
        self.x += self.dx * timer::delta(ctx).as_secs_f32();
        self.y += self.dy * timer::delta(ctx).as_secs_f32();

        if self.y <= 0.0 || self.y >= WINDOW_HEIGHT - BALL_RADIUS {
            self.dy = -self.dy;
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(self.x, self.y),
            BALL_RADIUS,
            0.1,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        graphics::draw(ctx, &circle, DrawParam::default())?;
        Ok(())
    }
}

struct GameState {
    paddle_left: Paddle,
    paddle_right: Paddle,
    ball: Ball,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        let paddle_left = Paddle::new(10.0, WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0);
        let paddle_right = Paddle::new(
            WINDOW_WIDTH - PADDLE_WIDTH - 10.0,
            WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
        );
        let ball = Ball::new(
            WINDOW_WIDTH / 2.0 - BALL_RADIUS / 2.0,
            WINDOW_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
            BALL_SPEED,
            BALL_SPEED,
        );

        Ok(GameState {
            paddle_left,
            paddle_right,
            ball,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.paddle_left.update(ctx);
        self.paddle_right.update(ctx);
        self.ball.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        self.paddle_left.draw(ctx)?;
        self.paddle_right.draw(ctx)?;
        self.ball.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
