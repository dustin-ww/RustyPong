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
// TODO - Implement Stick functions
impl Stick {
    fn new(position: Vec2, size: Vec2, movement_speed: f32) -> Stick { //Left/Right player? Whp controls what?
        Stick {
            position,
            size,
            movement_speed
        } // creates new Instance
    }

    fn translate(&mut self, direction: Dir, dt: f32)
    {
        match direction{
            Dir::UP => self.position.y = self.position.y - (self.movement_speed * dt),
            Dir::DOWN => self.position.y = self.position.y + (self.movement_speed * dt),
        }
    }

    fn update(&mut self, key_code: KeyCode, dt: f32) {
    }

    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE); //from macroquad lib, draws a rectangle at pos x, y and w, h
    }
}

struct Ball {
    position: Vec2, // I am using Vec2 bc Position is not native
    direction: Vec2,
    size: Vec2,
    movement_speed: f32,
}

// TODO - Implement Ball functions
impl Ball {
    fn new(position: Vec2, direction: Vec2, size: Vec2, movement_speed: f32) -> Ball {
        Ball{
            position,
            direction,
            size,
            movement_speed
        }
    }

    fn update(&mut self, delta_time: f32) {
    }

    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE);
    }
}

struct  GameState {
    score_left: i32,
    score_right: i32,
    is_running: bool,

    left_stick: Stick,
    right_stick: Stick,

}
impl GameState {
    fn new(left_stick: Stick, right_stick: Stick) -> GameState {
        GameState {
            score_left: 0,
            score_right: 0,
            is_running: true,
            left_stick,
            right_stick,
        }
    }

}




fn update(game_state: &mut GameState , dt: f32) {

    if is_key_down(KeyCode::W) {
        game_state.left_stick.translate(Dir::UP, dt);
    }
    else if is_key_down(KeyCode::S) {
        game_state.left_stick.translate(Dir::DOWN, dt);
    }

    
    // Player1 Input

    // Player2 Input


    //TODO Check right stick

    //Check Collision
    // Ball with stick
    // balls to the wall
    // ball with goal --> updateScore

    //checkfinished

}
/*
fn draw(gameState: &mut GameState , dt: f32) -> GameState {
    //        clear_background(BLACK);
    //         left_stick.draw();
    //         right_stick.draw();
    //         ball.draw();
    //          overlay.draw();
}
// 0.3 * window_width
*/

// TODO: 
// {} - draw sticks and ball onto screen
// {} - implement right_stick movement 
// {} - implement ball collisions (both with sticks and wall)
// {} - implement gameplay (also funktionen die bei gewissen collisionen aufgerufen werden z.B.)

#[macroquad::main("Pong")]
async fn main() {
    let stick_size = Vec2::new(20.0, 60.0); // Just guessing
    let ball_size = Vec2::new(20.0, 20.0);
    let left_stick = Stick::new(Vec2::new(10.0, 0.0), stick_size, 0.1); // Height based on screen height? y = screen_height /3 or so?
    let right_stick = Stick::new(Vec2::new(500.0, 0.0), stick_size, 0.1); // Hard coding the position isn't too good either

    // gameState(ball,left_stick,right_stick)
    
    // TODO - Update the positon of the ball to be in the middle an give an direction
    //let ball = Ball::new(Vec2::new((right_stick.position.x - left_stick.position.x) / 2.0 , (right_stick.position.y / 2.0) ), Vec2::new(0.0,0.0), ball_size); // Typesave: Cannot divide a f32 by an i32
    // No deltatime
 //   while gameState.is_running {
    
    let mut game_state = GameState::new(left_stick, right_stick);
    loop {
        let delta_time = get_frame_time();
        
        update(&mut game_state, delta_time);
        //  update(gameState)
        // draw()

        // Exit when set score or esc
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE); 
        next_frame().await;
    }
    // Show winner
}
