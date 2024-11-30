use macroquad::prelude::*;

// To run this file execute cargo run --bin game_window from root dir
struct Stick {
    position: Vec2, // center point of the rectangle
    size: Vec2,
}
// TODO - Implement Stick functions
impl Stick {
    fn new(position: Vec2, size: Vec2) -> Stick { //Left/Right player? Whp controls what?
        Stick {
            position,
            size
        } // creates new Instance
    }

    fn update() { // What inputs, keyboard inputs or position values? Or do we update somewhere else?
        todo!(); screen_height();
    }

    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE); //from macroquad lib, draws a rectangle at pos x, y and w, h
    }
}

struct Ball {
    position: Vec2, // I am using Vec2 bc Position is not native
    direction: Vec2,
    size: Vec2,
}

// TODO - Implement Ball functions
impl Ball {
    fn new(position: Vec2, direction: Vec2, size: Vec2) -> Ball {
        Ball{
            position,
            direction,
            size,
        }
    }

    fn update() { // Update Position based on Stick/Wall collision
        todo!();
        // Do we have to include the speed of the sticks when colliding?
    }

    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE);
    }
}

struct  GameState {
    score_left: i32,
    score_right: i32,
    is_running: bool,
}

impl GameState {
    fn new() -> GameState {
        GameState {

        }
    }

}

fn update(gameState: &mut GameState , dt: f32) -> GameState {

    if is_key_down(KeyCode::Down) {
        gameState.left_stick.update(KeyCode::Down, dt) // Muss sich letzte Position merken
    } else if is_key_down(KeyCode::Up) {
        gameState.left_stick.update(KeyCode::Up, dt) // Muss sich letzte Position merken
    }
    //TODO Check right stick

    //Check Collision
    // Ball with stick
    // balls to the wall
    // ball with goal --> updateScore

    //checkfinished

}

fn draw(gameState: &mut GameState , dt: f32) -> GameState {
    //        clear_background(BLACK);
    //         left_stick.draw();
    //         right_stick.draw();
    //         ball.draw();
    //          overlay.draw();
}

#[macroquad::main("Pong")]
async fn main() {
    let stick_size = Vec2::new(20.0, 60.0); // Just guessing
    let ball_size = Vec2::new(20.0, 20.0);
    let left_stick = Stick::new(Vec2::new(10.0, 0.0), stick_size); // Height based on screen height? y = screen_height /3 or so?
    let right_stick = Stick::new(Vec2::new(500.0, 0.0), stick_size); // Hard coding the position isn't too good either

    // gameState(ball,left_stick,right_stick)

    // TODO - Update the positon of the ball to be in the middle an give an direction
    let ball = Ball::new(Vec2::new((right_stick.position.x - left_stick.position.x) / 2.0 , (right_stick.position.y / 2.0) ), Vec2::new(0.0,0.0), ball_size); // Typesave: Cannot divide a f32 by an i32
    // dt
    while gameState.is_running {

        //  update(gameState, dt)
        // draw()

        // Exit when set score or esc
        next_frame().await;
    }
    // Show winner
}