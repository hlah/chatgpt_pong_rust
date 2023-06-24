use ggez::{
    event::{self, EventHandler},
    graphics, Context, GameResult,
};
use nalgebra::{Point2, Vector2};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 60.0;
const PADDLE_SPEED: f32 = 5.0;

struct Ball {
    position: Point2<f32>,
    velocity: Vector2<f32>,
}

struct Paddle {
    position: Point2<f32>,
    velocity: f32,
}

struct MainState {
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let ball = Ball {
            position: Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            velocity: Vector2::new(2.0, 2.0),
        };

        let player1 = Paddle {
            position: Point2::new(10.0, WINDOW_HEIGHT / 2.0),
            velocity: 0.0,
        };

        let player2 = Paddle {
            position: Point2::new(WINDOW_WIDTH - 10.0, WINDOW_HEIGHT / 2.0),
            velocity: 0.0,
        };

        Ok(MainState {
            ball,
            player1,
            player2,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.ball.position += self.ball.velocity;

        if self.ball.position.y < 0.0 || self.ball.position.y > WINDOW_HEIGHT {
            self.ball.velocity.y *= -1.0;
        }

        self.player1.position.y += self.player1.velocity;
        self.player2.position.y += self.player2.velocity;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(0.0, 0.0),
            10.0,
            2.0,
            graphics::WHITE,
        )?;

        let paddle = graphics::Rect::new(
            -PADDLE_WIDTH / 2.0,
            -PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
        let paddle1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            paddle,
            graphics::WHITE,
        )?;
        let paddle2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            paddle,
            graphics::WHITE,
        )?;

        graphics::draw(
            ctx,
            &ball,
            graphics::DrawParam::default().dest(self.ball.position),
        )?;
        graphics::draw(
            ctx,
            &paddle1,
            graphics::DrawParam::default().dest(self.player1.position),
        )?;
        graphics::draw(
            ctx,
            &paddle2,
            graphics::DrawParam::default().dest(self.player2.position),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.player1.position.y = y;
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
