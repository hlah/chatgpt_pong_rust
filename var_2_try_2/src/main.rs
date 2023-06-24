extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Point3, Vector2};

const PADDLE_WIDTH: f32 = 0.2;
const PADDLE_HEIGHT: f32 = 1.0;
const BALL_RADIUS: f32 = 0.1;

struct Paddle {
    node: SceneNode,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Paddle {
        let mut node = Window::new("Pong").add_cube(PADDLE_WIDTH, PADDLE_HEIGHT, 0.1);
        node.set_color(0.0, 1.0, 0.0);

        Paddle {
            node,
            position: Vector2::new(x, y),
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.node
            .set_local_translation(Point3::new(self.position.x, self.position.y, 0.0));
    }
}

struct Ball {
    node: SceneNode,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Ball {
    fn new(x: f32, y: f32) -> Ball {
        let mut node = Window::new("Pong").add_sphere(BALL_RADIUS);
        node.set_color(1.0, 0.0, 0.0);

        Ball {
            node,
            position: Vector2::new(x, y),
            velocity: Vector2::new(0.1, 0.1),
        }
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.node
            .set_local_translation(Point3::new(self.position.x, self.position.y, 0.0));
    }
}

fn main() {
    let mut window = Window::new("Pong");
    window.set_light(Light::StickToCamera);

    let mut left_paddle = Paddle::new(-6.0, 0.0);
    let mut right_paddle = Paddle::new(6.0, 0.0);
    let mut ball = Ball::new(0.0, 0.0);

    while window.render() {
        let dt = window.scene_mut().time().delta_seconds();

        if window.get_key(Key::W) == Action::Press {
            left_paddle.velocity.y = 1.0;
        } else if window.get_key(Key::S) == Action::Press {
            left_paddle.velocity.y = -1.0;
        } else {
            left_paddle.velocity.y = 0.0;
        }

        if window.get_key(Key::Up) == Action::Press {
            right_paddle.velocity.y = 1.0;
        } else if window.get_key(Key::Down) == Action::Press {
            right_paddle.velocity.y = -1.0;
        } else {
            right_paddle.velocity.y = 0.0;
        }

        left_paddle.update(dt);
        right_paddle.update(dt);
        ball.update(dt);
    }
}
