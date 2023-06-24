use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::{Point2, Vector2};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 15.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;

struct Paddle {
    position: Point2<f32>,
    size: Vector2<f32>,
    velocity: f32,
}

impl Paddle {
    fn new(x: f32) -> Paddle {
        Paddle {
            position: Point2::new(x, WINDOW_HEIGHT / 2.0),
            size: Vector2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            velocity: 0.0,
        }
    }

    fn update(&mut self) {
        self.position.y += self.velocity;
        self.position.y = self.position.y.min(WINDOW_HEIGHT - self.size.y / 2.0);
        self.position.y = self.position.y.max(self.size.y / 2.0);
    }
}

struct Ball {
    position: Point2<f32>,
    velocity: Vector2<f32>,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            position: Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            velocity: Vector2::new(-BALL_SPEED, BALL_SPEED),
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;

        if self.position.y <= BALL_RADIUS || self.position.y >= WINDOW_HEIGHT - BALL_RADIUS {
            self.velocity.y = -self.velocity.y;
        }
    }
}

fn main() {
    let mut window = Window::new("Pong Clone");
    window.set_light(Light::StickToCamera);

    let mut left_paddle = Paddle::new(10.0);
    let mut right_paddle = Paddle::new(WINDOW_WIDTH - 10.0);
    let mut ball = Ball::new();

    while window.render() {
        if window.get_key(Key::Up) == Action::Press {
            right_paddle.velocity = -PADDLE_SPEED;
        } else if window.get_key(Key::Down) == Action::Press {
            right_paddle.velocity = PADDLE_SPEED;
        } else {
            right_paddle.velocity = 0.0;
        }

        if window.get_key(Key::W) == Action::Press {
            left_paddle.velocity = -PADDLE_SPEED;
        } else if window.get_key(Key::S) == Action::Press {
            left_paddle.velocity = PADDLE_SPEED;
        } else {
            left_paddle.velocity = 0.0;
        }

        left_paddle.update();
        right_paddle.update();
        ball.update();

        window.draw_line(
            &Point2::new(WINDOW_WIDTH / 2.0, 0.0),
            &Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT),
            &kiss3d::window::LineOptions::new(1.0, nalgebra::Vector3::new(1.0, 1.0, 1.0)),
        );

        window.draw_rectangle(
            &kiss3d::planar_camera::Rect::new(
                left_paddle.position.x - left_paddle.size.x / 2.0,
                left_paddle.position.y - left_paddle.size.y / 2.0,
                left_paddle.size.x,
                left_paddle.size.y,
            ),
            &kiss3d::window::FillColor::Rgba(0.0, 1.0, 0.0, 1.0),
        );

        window.draw_rectangle(
            &kiss3d::planar_camera::Rect::new(
                right_paddle.position.x - right_paddle.size.x / 2.0,
                right_paddle.position.y - right_paddle.size.y / 2.0,
                right_paddle.size.x,
                right_paddle.size.y,
            ),
            &kiss3d::window::FillColor::Rgba(0.0, 0.0, 1.0, 1.0),
        );

        window.draw_circle(
            &kiss3d::planar_camera::Circle::new(ball.position, BALL_RADIUS),
            &kiss3d::window::FillColor::Rgba(1.0, 1.0, 0.0, 1.0),
        );
    }
}
