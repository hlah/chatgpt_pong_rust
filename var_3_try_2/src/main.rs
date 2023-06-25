extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::event::{Action, Key};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, Translation3, Vector3};

const PADDLE_WIDTH: f32 = 0.1;
const PADDLE_HEIGHT: f32 = 0.4;
const BALL_RADIUS: f32 = 0.1;
const BALL_SPEED: f32 = 0.01;
const PADDLE_SPEED: f32 = 0.02;

fn main() {
    let mut window = Window::new("Pong");
    let mut world = window.scene_mut();

    world.add(Light::StickToCamera);

    let mut paddle1 = world.add_cube(PADDLE_WIDTH, PADDLE_HEIGHT, 0.01);
    paddle1.set_color(1.0, 0.0, 0.0);
    paddle1.append_translation(&Translation3::new(-1.0, 0.0, 0.0));

    let mut paddle2 = world.add_cube(PADDLE_WIDTH, PADDLE_HEIGHT, 0.01);
    paddle2.set_color(0.0, 0.0, 1.0);
    paddle2.append_translation(&Translation3::new(1.0, 0.0, 0.0));

    let mut ball = world.add_sphere(BALL_RADIUS);
    ball.set_color(1.0, 1.0, 1.0);
    ball.append_translation(&Translation3::new(0.0, 0.0, 0.0));

    let mut ball_velocity = Vector3::new(BALL_SPEED, BALL_SPEED, 0.0);

    while window.render() {
        handle_input(&mut paddle1, &mut paddle2);

        let ball_pos = ball.translation().vector;
        ball.append_translation(&Translation3::from(ball_velocity));

        if ball_pos.y >= 1.0 || ball_pos.y <= -1.0 {
            ball_velocity.y *= -1.0;
        }

        if ball_pos.x >= 1.0 || ball_pos.x <= -1.0 {
            ball.append_translation(&Translation3::new(-ball_pos.x, -ball_pos.y, 0.0));
        }

        if ball.intersects(&paddle1) || ball.intersects(&paddle2) {
            ball_velocity.x *= -1.0;
        }
    }
}

fn handle_input(paddle1: &mut kiss3d::scene::SceneNode, paddle2: &mut kiss3d::scene::SceneNode) {
    if kiss3d::event::is_key_down(Key::Up) {
        paddle1.prepend_to_local_translation(&Translation3::new(0.0, PADDLE_SPEED, 0.0));
    }

    if kiss3d::event::is_key_down(Key::Down) {
        paddle1.prepend_to_local_translation(&Translation3::new(0.0, -PADDLE_SPEED, 0.0));
    }

    if kiss3d::event::is_key_down(Key::W) {
        paddle2.prepend_to_local_translation(&Translation3::new(0.0, PADDLE_SPEED, 0.0));
    }

    if kiss3d::event::is_key_down(Key::S) {
        paddle2.prepend_to_local_translation(&Translation3::new(0.0, -PADDLE_SPEED, 0.0));
    }
}
