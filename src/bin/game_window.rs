use ::rand::{thread_rng, Rng};
use macroquad::miniquad::gl::glActiveTexture;
use macroquad::prelude::*;

enum Dir {
    UP,
    DOWN,
}
// To run this file execute cargo run --bin game_window from root dir
struct Stick {
    position: Vec2, // center point of the rectangle
    size: Vec2,
    movement_speed: f32,
}

impl Stick {
    fn new(position: Vec2, size: Vec2, movement_speed: f32) -> Stick {
        Stick {
            position,
            size,
            movement_speed,
        } // creates new Instance
    }

    fn translate(&mut self, direction: Dir, dt: f32) {
        match direction {
            Dir::UP => {
                if self.position.y > 0.0 {
                    self.position.y = self.position.y - (self.movement_speed * dt)
                }
            }
            Dir::DOWN => {
                if self.position.y < screen_height() - self.size.y {
                    self.position.y = self.position.y + (self.movement_speed * dt)
                }
            }
        }
    }

    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            WHITE,
        ); //from macroquad lib, draws a rectangle at pos x, y and w, h
    }
}

struct Ball {
    position: Vec2, // I am using Vec2 bc Position is not native
    direction: Vec2,
    radius: f32,
    movement_speed: f32,
}

impl Ball {
    fn new(position: Vec2, mut direction: Vec2, radius: f32, movement_speed: f32) -> Ball {
        Ball {
            position,
            direction,
            radius,
            movement_speed,
        }
    }

    fn reset(&mut self) {
        let mut rng = thread_rng();
        let start_direction_x = rng.gen_range(-1.0..1.0);
        let start_direction_y = rng.gen_range(-1.0..1.0);

        self.direction = Vec2::new(start_direction_x, start_direction_y);

        self.position = Vec2::new(
            (screen_width() / 2.0) - self.radius,
            (screen_height() / 2.0) - self.radius,
        )
    }

    fn translate(&mut self, dt: f32) {
        // Normalize the direction to ensure consistent speed
        if self.direction.length() > 0.0 {
            self.direction = self.direction.normalize();
        }

        self.position.x += self.direction.x * (dt * self.movement_speed);
        self.position.y += self.direction.y * (dt * self.movement_speed);
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }
}

struct GameState {
    score_left: i32,
    score_right: i32,
    is_running: bool,

    left_stick: Stick,
    right_stick: Stick,

    ball: Ball,
}
impl GameState {
    fn new(left_stick: Stick, right_stick: Stick, ball: Ball) -> GameState {
        GameState {
            score_left: 0,
            score_right: 0,
            is_running: true,
            left_stick,
            right_stick,
            ball,
        }
    }

    fn reset(&mut self) {
        self.score_left = 0;
        self.score_right = 0;
        self.is_running = true;
    }
}

fn check_collision(game_state: &mut GameState) {
    // Wall collision (reverse y direction)
    if game_state.ball.position.y - game_state.ball.radius <= 10.0
        || game_state.ball.position.y + game_state.ball.radius >= screen_height() - 10.0
    {
        game_state.ball.direction.y = -game_state.ball.direction.y;
    }

    // Stick collision (reverse x direction)
    if game_state.ball.position.x - game_state.ball.radius
        <= game_state.left_stick.position.x + game_state.left_stick.size.x
        && game_state.ball.position.y >= game_state.left_stick.position.y
        && game_state.ball.position.y
            <= game_state.left_stick.position.y + game_state.left_stick.size.y
    {
        game_state.ball.direction.x = -game_state.ball.direction.x;
    } else if game_state.ball.position.x + game_state.ball.radius
        >= game_state.right_stick.position.x
        && game_state.ball.position.y >= game_state.right_stick.position.y
        && game_state.ball.position.y
            <= game_state.right_stick.position.y + game_state.right_stick.size.y
    {
        game_state.ball.direction.x = -game_state.ball.direction.x;
    }

    // Goal
    if game_state.ball.position.x - game_state.ball.radius < 0.0 {
        game_state.score_right += 1;
        game_state.ball.reset();
    } else if game_state.ball.position.x + game_state.ball.radius > screen_width() {
        game_state.score_left += 1;
        game_state.ball.reset();
    }
}

fn update(game_state: &mut GameState, dt: f32) {
    if is_key_down(KeyCode::Escape) {
        game_state.is_running = false;
    }
    if is_key_pressed(KeyCode::R) {
        game_state.reset();
        game_state.ball.reset();
    }

    if is_key_down(KeyCode::W) {
        game_state.left_stick.translate(Dir::UP, dt);
    } else if is_key_down(KeyCode::S) {
        game_state.left_stick.translate(Dir::DOWN, dt);
    }

    if is_key_down(KeyCode::Up) {
        game_state.right_stick.translate(Dir::UP, dt);
    } else if is_key_down(KeyCode::Down) {
        game_state.right_stick.translate(Dir::DOWN, dt);
    }

    game_state.ball.translate(dt);

    // Check Collision
    check_collision(game_state);

    if game_state.score_right >= 5 || game_state.score_left >= 5 {
        game_state.is_running = false;
    }
}

fn draw(game_state: &mut GameState, dt: f32) {
    clear_background(BLACK);

    draw_rectangle(30.0, 0.0, 840.0, 10.0, GRAY);
    draw_rectangle(30.0, 390.0, 840.0, 10.0, GRAY);
    game_state.left_stick.draw();
    game_state.right_stick.draw();

    game_state.ball.draw();

    //draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);
    let score = format!(
        "{left} : {right}",
        left = game_state.score_left,
        right = game_state.score_right
    );
    let font_size = 60.0;
    let size = measure_text(score.as_str(), None, font_size as _, 1.0);
    draw_text(
        score.as_str(),
        screen_width() / 2.0 - size.width / 2.0 - 10.0,
        screen_height() / 2.0,
        font_size,
        GRAY,
    );
    let restart = "PRESS [R] TO RESTART";
    let font_size_restart = 20.0;
    let size_restart = measure_text(restart, None, font_size_restart as _, 1.0);
    draw_text(
        restart,
        screen_width() / 2.0 - size_restart.width / 2.0 - 10.0,
        screen_height() / 2.0 + size.height + 10.0,
        font_size_restart,
        GRAY,
    );
}

// TODO:
// {x} - draw sticks and ball onto screen
// {x} - implement right_stick movement
// {x} - implement ball movement
// {x} - implement ball collisions (both with sticks and wall)
// {x} - implement gameplay (also funktionen die bei gewissen collisionen aufgerufen werden z.B.)
// {} - Show winner
// {} - test collisions
// {} - bissi schön machen?

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_title: "RustyPong".to_string(),
        window_height: 400,
        window_width: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let stick_movementspeed = 150.0;
    let stick_size = Vec2::new(20.0, 60.0);
    let ball_radius = 10.0;
    let left_stick = Stick::new(
        Vec2::new(10.0, screen_height() / 2.0 - stick_size.y / 2.0),
        stick_size,
        stick_movementspeed,
    );
    let right_stick = Stick::new(
        Vec2::new(870.0, screen_height() / 2.0 - stick_size.y / 2.0),
        stick_size,
        stick_movementspeed,
    ); // Hard coding the position isn't too good either

    let ball = Ball::new(
        Vec2::new(
            (screen_width() / 2.0) - ball_radius / 2.0,
            (screen_height() / 2.0) - ball_radius / 2.0,
        ),
        Vec2::ZERO,
        ball_radius,
        200.0,
    ); // Typesave: Cannot divide a f32 by an i32

    let mut game_state = GameState::new(left_stick, right_stick, ball);
    game_state.ball.reset();
    while game_state.is_running {
        let delta_time = get_frame_time();

        update(&mut game_state, delta_time);
        draw(&mut game_state, delta_time);

        next_frame().await;
    }
}
