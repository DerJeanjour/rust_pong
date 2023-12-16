use piston_window::*;
use rand::Rng;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;
const WINDOW_FPS : u64 = 60;

const RED : [f32; 4] = [1.0,0.0,0.0,1.0];
const GREEN : [f32; 4] = [0.0,1.0,0.0,1.0];
const BLUE : [f32; 4] = [0.0,0.0,1.0,1.0];

const BALL_RADIUS : f64 = 10.0;
const BALL_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BALL_VELOCITY_INC : f64 = 1.5;

const PADDLE_WIDTH : f64 = 10.0;
const PADDLE_HEIGHT : f64 = 160.0;
const PADDLE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PADDLE_VELOCITY_INC : f64 = 2.0;

const BACKGROUND_COLOR : [f32; 4] = [0.1, 0.1, 0.1, 1.0];

const ZERO_VEC : Vec2f = Vec2f { x: 0.0, y: 0.0 };

#[derive(Clone)]
struct Vec2f {
    x: f64,
    y: f64,
}

impl Vec2f {
    fn length(&self) -> f64 {
        ( self.x.powi(2) + self.y.powi(2) ).sqrt()
    }
    fn normalize(&self) -> Vec2f {
        let len = self.length();
        if len <= 0.0 {
            return ZERO_VEC.clone();
        }
        Vec2f { x: self.x / len, y: self.y / len }
    }
    fn add(&self, vec : &Vec2f) -> Vec2f {
        Vec2f { x: self.x + vec.x, y: self.y + vec.y }
    }
    fn dot(&self, vec : &Vec2f) -> f64 {
        self.x * vec.x + self.y * vec.y
    }
    fn to_array(&self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl ToString for Vec2f {
    fn to_string(&self) -> String {
        format!("({:.2}, {:.2})", self.x, self.y)
    }
}


struct BBox {
    min: Vec2f,
    max: Vec2f
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Direction {
    fn vector(&self) -> Vec2f {
        match self {
            Direction::UP => Vec2f { x: 0.0, y: -1.0 },
            Direction::DOWN => Vec2f { x: 0.0, y: 1.0 },
            Direction::LEFT => Vec2f { x: -1.0, y: 0.0 },
            Direction::RIGHT => Vec2f { x: 1.0, y: 0.0 },
        }
    }
}

#[derive(Clone)]
enum PlayerType {
    HUMAN,
    BOT
}

trait GameElement {
    fn get_bbox(&self) -> BBox;
    fn draw(&self, context: &Context, g2d: &mut G2d);
}

#[derive(Clone)]
struct Paddle {
    pos: Vec2f,
    size: Vec2f,
    velocity: Vec2f,
    player_type: PlayerType
}

impl Paddle {
    fn is_human(&self) -> bool {
        matches!(self.player_type, PlayerType::HUMAN)
    }
}

impl GameElement for Paddle {
    fn get_bbox(&self) -> BBox {
        BBox { min: Vec2f { x: self.pos.x, y: self.pos.y }, max: Vec2f { x: self.pos.x + self.size.x, y: self.pos.y + self.size.y } }
    }
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle(PADDLE_COLOR,
            [self.pos.x, self.pos.y, self.size.x, self.size.y],
            context.transform, g2d);
    }
}

#[derive(Clone)]
struct Ball {
    pos: Vec2f,
    radius: f64,
    velocity: Vec2f
}

impl GameElement for Ball {
    fn get_bbox(&self) -> BBox {
        BBox { min: Vec2f { x: self.pos.x-self.radius, y: self.pos.y-self.radius }, max: Vec2f { x: self.pos.x+self.radius, y: self.pos.y+self.radius } }
    }
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        ellipse(BALL_COLOR,
            [self.pos.x - self.radius, self.pos.y - self.radius, self.radius * 2.0, self.radius * 2.0],
            context.transform, g2d);
    }
}

struct GameState {
    ball: Ball,
    paddle_left: Paddle,
    paddle_right: Paddle
}

impl GameState {
    fn get_ball(&mut self) -> &mut Ball {
        &mut self.ball
    }
    fn get_paddle_left(&mut self) -> &mut Paddle {
        &mut self.paddle_left
    }
    fn get_paddle_right(&mut self) -> &mut Paddle {
        &mut self.paddle_right
    }
    fn set_human_paddle_velocity(&mut self, velocity_y: f64) {
        if self.paddle_left.is_human() {
            self.paddle_left.velocity.y = velocity_y;
        }
        if self.paddle_right.is_human() {
            self.paddle_right.velocity.y = velocity_y;
        }
    }
    fn set_bot_paddle_velocity(&mut self, velocity_y: f64) {
        if !self.paddle_left.is_human() {
            self.paddle_left.velocity.y = velocity_y;
        }
        if !self.paddle_right.is_human() {
            self.paddle_right.velocity.y = velocity_y;
        }
    }
}

fn main() {

    let mut window: PistonWindow = WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(WINDOW_FPS);
    window.set_ups(WINDOW_FPS);

    let mut game_state = GameState { 
        ball: Ball { 
            pos: ZERO_VEC, 
            radius: BALL_RADIUS, 
            velocity: ZERO_VEC
        },
        paddle_left: Paddle { 
            pos: Vec2f { x: 100.0, y: 250.0 }, 
            size: Vec2f { x: PADDLE_WIDTH, y: PADDLE_HEIGHT }, 
            velocity: Vec2f {  x: 0.0, y: 0.0 }, 
            player_type: PlayerType::HUMAN 
        },
        paddle_right: Paddle { 
            pos: Vec2f { x: 700.0, y: 250.0 }, 
            size: Vec2f { x: PADDLE_WIDTH, y: PADDLE_HEIGHT }, 
            velocity: Vec2f {  x: 0.0, y: 0.0 }, 
            player_type: PlayerType::HUMAN 
        }
    };

    new_round(&mut game_state);
    while let Some( event) = window.next() {

        handle_input(&event, &mut game_state);
        handle_bot(&mut game_state);
        let reset = update_game(&mut game_state);
        if reset {
            new_round(&mut game_state);
        }

        window.draw_2d(&event, |context, g2d, _| {

            clear(BACKGROUND_COLOR, g2d);

            game_state.get_paddle_left().draw(&context, g2d);
            game_state.get_paddle_right().draw(&context, g2d);
            game_state.get_ball().draw(&context, g2d);
        });

    }
}

fn new_round( game_state : &mut GameState  ) {
    let mut rng = rand::thread_rng();
    game_state.ball.pos = Vec2f { x: ( WINDOW_WIDTH / 2 ) as f64, y: ( WINDOW_HEIGHT / 2 ) as f64 };
    game_state.ball.velocity = Vec2f { 
        x: rng.gen_range(-1.0..1.0) * BALL_VELOCITY_INC, 
        y: rng.gen_range(-1.0..1.0) * BALL_VELOCITY_INC 
    };
}

fn handle_input(event: &Event, game_state: &mut GameState) {

    // on key press
    if let Some(Button::Keyboard(key)) = event.press_args() {
        match key {
            Key::Up => game_state.set_human_paddle_velocity(-PADDLE_VELOCITY_INC),
            Key::Down => game_state.set_human_paddle_velocity(PADDLE_VELOCITY_INC),
            _ => {}
        }
    }

    // on key release
    if let Some(Button::Keyboard(key)) = event.release_args() {
        match key {
            Key::Up => game_state.set_human_paddle_velocity(0.0),
            Key::Down => game_state.set_human_paddle_velocity(0.0),
            _ => {}
        }
    }

}

fn handle_bot(game_state: &mut GameState) {
    // TODO
}

fn update_game(game_state: &mut GameState) -> bool {

    // immutable game elements
    let ball = game_state.get_ball().clone();
    let left_paddle = game_state.get_paddle_left().clone();
    let right_paddle = game_state.get_paddle_right().clone();

    // move paddle
    const max_h : f64 = WINDOW_HEIGHT as f64;
    game_state.get_paddle_left().pos.y = ( left_paddle.pos.y + left_paddle.velocity.y ).clamp( 0.0, max_h - left_paddle.size.y );
    game_state.get_paddle_right().pos.y = ( right_paddle.pos.y + right_paddle.velocity.y ).clamp( 0.0, max_h - right_paddle.size.y );

    

    // handle ball out of bounds on width
    if is_out_of_bounds_on_width( &ball.get_bbox() ) {
        return true; // early break, signal new round
    }

    let mut bounce_direction = ball.velocity.normalize();

    // handle ball out of bounds on height
    if is_out_of_bounds_on_height( &ball.get_bbox() ) {
        bounce_direction.y *= -1.0;
    }

    // handle collision with left paddle
    if has_collision(&ball.get_bbox(), &left_paddle.get_bbox()) {
        bounce_direction = reflect(&ball.velocity, &Direction::RIGHT.vector());
    }

    // handle collision with right paddle
    if has_collision(&ball.get_bbox(), &right_paddle.get_bbox()) {
        bounce_direction = reflect(&ball.velocity, &Direction::LEFT.vector());
    }

    // update ball velocity
    game_state.get_ball().velocity.x = bounce_direction.x * BALL_VELOCITY_INC;
    game_state.get_ball().velocity.y = bounce_direction.y * BALL_VELOCITY_INC;
    //game_state.get_ball().velocity = ball_velocity;

    // move ball
    game_state.get_ball().pos.x += game_state.get_ball().velocity.x;
    game_state.get_ball().pos.y += game_state.get_ball().velocity.y;

    false
}

// ----- UTILS -----

fn is_out_of_bounds(bbox: &BBox) -> bool {
    is_out_of_bounds_on_width(bbox) || is_out_of_bounds_on_height(bbox)
}

fn is_out_of_bounds_on_width(bbox: &BBox) -> bool {
    let max_w = WINDOW_WIDTH as f64;
    bbox.min.x < 0.0 || bbox.max.x > max_w
}

fn is_out_of_bounds_on_height(bbox: &BBox) -> bool {
    let max_h = WINDOW_HEIGHT as f64;
    bbox.min.y < 0.0 || bbox.max.y > max_h
}

fn has_collision(bbox_a: &BBox, bbox_b: &BBox) -> bool {
    bbox_a.min.x <= bbox_b.max.x && bbox_a.max.x >= bbox_b.min.x && bbox_a.min.y <= bbox_b.max.y && bbox_a.max.y >= bbox_b.min.y
}

fn reflect(dir: &Vec2f, normal: &Vec2f) -> Vec2f {
    let dot_product = dir.dot(normal);
    Vec2f {
        x: dir.x - 2.0 * dot_product * normal.x,
        y: dir.y - 2.0 * dot_product * normal.y,
    }.normalize()
}