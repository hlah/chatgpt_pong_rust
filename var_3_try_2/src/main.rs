use kiss3d::event::{Action, Key};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::{Point3, Translation3, Vector3};

const PADDLE_SPEED: f32 = 0.1;
const BALL_VELOCITY: Vector3<f32> = Vector3::new(0.004, 0.002, 0.0);

fn main() {
    let mut window = Window::new("Pong");
    let mut world = window.scene_mut();

    let mut paddle1 = world.add_cube(0.2, 0.04, 0.04);
    paddle1.set_color(1.0, 0.0, 0.0);
    paddle1.append_translation(&Translation3::new(-1.0, 0.0, 0.0));

    let mut paddle2 = world.add_cube(0.2, 0.04, 0.04);
    paddle2.set_color(0.0, 0.0, 1.0);
    paddle2.append_translation(&Translation3::new(1.0, 0.0, 0.0));

    let mut ball = world.add_sphere(0.03);
    ball.set_color(1.0, 1.0, 1.0);
    ball.append_translation(&Translation3::new(0.0, 0.0, 0.0));

    world.add(Light::StickToCamera);

    let mut ball_pos = Point3::origin();
    let mut ball_velocity = BALL_VELOCITY;

    while window.render() {
        if window.get_key_down(Key::Up) {
            paddle1.prepend_to_local_translation(&Translation3::new(0.0, PADDLE_SPEED, 0.0));
        }
        if window.get_key_down(Key::Down) {
            paddle1.prepend_to_local_translation(&Translation3::new(0.0, -PADDLE_SPEED, 0.0));
        }
        if window.get_key_down(Key::W) {
            paddle2.prepend_to_local_translation(&Translation3::new(0.0, PADDLE_SPEED, 0.0));
        }
        if window.get_key_down(Key::S) {
            paddle2.prepend_to_local_translation(&Translation3::new(0.0, -PADDLE_SPEED, 0.0));
        }

        ball_pos += ball_velocity;
        ball.set_local_translation(Translation3::from(ball_pos));

        if ball_pos.y <= -0.95 || ball_pos.y >= 0.95 {
            ball_velocity.y = -ball_velocity.y;
        }

        if ball.intersects(&paddle1) || ball.intersects(&paddle2) {
            ball_velocity.x = -ball_velocity.x;
        }
    }
}
