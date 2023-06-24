extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::env;
use std::path;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 400.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 200.0;

struct Paddle {
    position: na::Point2<f32>,
    velocity: f32,
}

struct Ball {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
}

struct MainState {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let player1 = Paddle {
            position: na::Point2::new(10.0, WINDOW_HEIGHT / 2.0),
            velocity: 0.0,
        };

        let player2 = Paddle {
            position: na::Point2::new(WINDOW_WIDTH - 10.0, WINDOW_HEIGHT / 2.0),
            velocity: 0.0,
        };

        let ball = Ball {
            position: na::Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            velocity: na::Vector2::new(-BALL_SPEED, -BALL_SPEED),
        };

        let s = MainState {
            player1,
            player2,
            ball,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(_ctx).as_secs_f32();

        // Movimento das paletas
        self.player1.position.y += self.player1.velocity * dt;
        self.player2.position.y += self.player2.velocity * dt;

        // Movimento da bola
        self.ball.position += self.ball.velocity * dt;

        // Colisão com as paredes superior e inferior
        if self.ball.position.y < BALL_RADIUS || self.ball.position.y > WINDOW_HEIGHT - BALL_RADIUS
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Colisão com as paletas
        if self.ball.position.x < PADDLE_WIDTH + BALL_RADIUS
            && (self.ball.position.y > self.player1.position.y - PADDLE_HEIGHT / 2.0
                && self.ball.position.y < self.player1.position.y + PADDLE_HEIGHT / 2.0)
        {
            self.ball.velocity.x = -self.ball.velocity.x;
        }

        if self.ball.position.x > WINDOW_WIDTH - PADDLE_WIDTH - BALL_RADIUS
            && (self.ball.position.y > self.player2.position.y - PADDLE_HEIGHT / 2.0
                && self.ball.position.y < self.player2.position.y + PADDLE_HEIGHT / 2.0)
        {
            self.ball.velocity.x = -self.ball.velocity.x;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            BALL_RADIUS,
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

        let draw_param = graphics::DrawParam::default();

        graphics::draw(ctx, &ball, (self.ball.position, draw_param))?;
        graphics::draw(ctx, &paddle1, (self.player1.position, draw_param))?;
        graphics::draw(ctx, &paddle2, (self.player2.position, draw_param))?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::W => self.player1.velocity = -PADDLE_SPEED,
            event::KeyCode::S => self.player1.velocity = PADDLE_SPEED,
            event::KeyCode::Up => self.player2.velocity = -PADDLE_SPEED,
            event::KeyCode::Down => self.player2.velocity = PADDLE_SPEED,
            _ => (),
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
    ) {
        match keycode {
            event::KeyCode::W | event::KeyCode::S => self.player1.velocity = 0.0,
            event::KeyCode::Up | event::KeyCode::Down => self.player2.velocity = 0.0,
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("pong", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
