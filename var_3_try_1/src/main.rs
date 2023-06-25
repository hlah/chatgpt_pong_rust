extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::window::Window;
use na::{Point2, Vector2};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;

struct Pong {
    window: Window,
    paddle1_pos: f32,
    paddle2_pos: f32,
    ball_pos: Point2<f32>,
    ball_direction: Vector2<f32>,
}

impl Pong {
    fn new() -> Pong {
        let mut window = Window::new("Pong");
        window.set_framerate_limit(Some(60));
        Pong {
            window,
            paddle1_pos: WINDOW_HEIGHT / 2.0,
            paddle2_pos: WINDOW_HEIGHT / 2.0,
            ball_pos: Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            ball_direction: Vector2::new(-1.0, 1.0).normalize() * BALL_SPEED,
        }
    }

    fn handle_input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(Key::W, Action::Hold, _) => self.paddle1_pos -= 5.0,
            WindowEvent::Key(Key::S, Action::Hold, _) => self.paddle1_pos += 5.0,
            WindowEvent::Key(Key::Up, Action::Hold, _) => self.paddle2_pos -= 5.0,
            WindowEvent::Key(Key::Down, Action::Hold, _) => self.paddle2_pos += 5.0,
            _ => (),
        }
    }

    fn update(&mut self) {
        self.ball_pos += self.ball_direction;

        if self.ball_pos.y <= 0.0 || self.ball_pos.y >= WINDOW_HEIGHT {
            self.ball_direction.y = -self.ball_direction.y;
        }

        if self.ball_pos.x <= 0.0 {
            self.reset_ball();
        }

        if self.ball_pos.x >= WINDOW_WIDTH {
            self.reset_ball();
        }

        if self.ball_pos.x <= PADDLE_WIDTH
            && self.ball_pos.y >= self.paddle1_pos
            && self.ball_pos.y <= self.paddle1_pos + PADDLE_HEIGHT
        {
            self.ball_direction.x = -self.ball_direction.x;
        }

        if self.ball_pos.x >= WINDOW_WIDTH - PADDLE_WIDTH
            && self.ball_pos.y >= self.paddle2_pos
            && self.ball_pos.y <= self.paddle2_pos + PADDLE_HEIGHT
        {
            self.ball_direction.x = -self.ball_direction.x;
        }
    }

    fn draw(&mut self) {
        self.window.draw_line(
            &Point2::new(WINDOW_WIDTH / 2.0, 0.0),
            &Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT),
            &Point2::new(1.0, 1.0),
        );

        self.window.draw_rectangle(
            &Point2::new(PADDLE_WIDTH, self.paddle1_pos),
            &Point2::new(
                PADDLE_WIDTH + PADDLE_HEIGHT,
                self.paddle1_pos + PADDLE_HEIGHT,
            ),
            &Point2::new(1.0, 1.0),
        );

        self.window.draw_rectangle(
            &Point2::new(WINDOW_WIDTH - PADDLE_WIDTH, self.paddle2_pos),
            &Point2::new(
                WINDOW_WIDTH - (PADDLE_WIDTH + PADDLE_HEIGHT),
                self.paddle2_pos + PADDLE_HEIGHT,
            ),
            &Point2::new(1.0, 1.0),
        );

        self.window.draw_circle(
            &Point2::new(self.ball_pos.x, self.ball_pos.y),
            BALL_RADIUS,
            &Point2::new(1.0, 1.0),
        );
    }

    fn reset_ball(&mut self) {
        self.ball_pos = Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        self.ball_direction = Vector2::new(-1.0, 1.0).normalize() * BALL_SPEED;
    }

    fn run(&mut self) {
        while self.window.render() {
            for event in self.window.events().iter() {
                match event.value {
                    WindowEvent::Key(key, action, _) => {
                        if action == Action::Press && key == Key::Escape {
                            return;
                        }
                        self.handle_input(&event.value);
                    }
                    _ => (),
                }
            }
            self.update();
            self.draw();
        }
    }
}

fn main() {
    let mut pong = Pong::new();
    pong.run();
}
