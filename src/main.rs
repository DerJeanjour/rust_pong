use piston_window::{*, types::Vec2d};

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;
const WINDOW_FPS : u64 = 60;

const BALL_RADIUS : f64 = 10.0;
const BALL_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const PADDLE_WIDTH : f64 = 10.0;
const PADDLE_HEIGHT : f64 = 80.0;
const PADDLE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PADDLE_VELOCITY_INC : f64 = 2.0;

const BACKGROUND_COLOR : [f32; 4] = [0.1, 0.1, 0.1, 1.0];

enum PlayerType {
    HUMAN,
    BOT
}

struct Vec2f {
    x: f64,
    y: f64,
}

struct BBox {
    ll: Vec2f,
    ur: Vec2f
}

trait GameElement {
    fn get_bbox(&self) -> BBox;
    fn draw(&self, context: &Context, g2d: &mut G2d);
}

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
        BBox { ll: Vec2f { x: self.pos.x, y: self.pos.y }, ur: Vec2f { x: self.pos.x + self.size.x, y: self.pos.y + self.size.y } }
    }
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle(PADDLE_COLOR,
            [self.pos.x, self.pos.y, self.size.x, self.size.y],
            context.transform, g2d);
    }
}

struct Ball {
    pos: Vec2f,
    radius: f64,
    velocity: Vec2f
}

impl GameElement for Ball {
    fn get_bbox(&self) -> BBox {
        BBox { ll: Vec2f { x: self.pos.x-self.radius, y: self.pos.y-self.radius }, ur: Vec2f { x: self.pos.x+self.radius, y: self.pos.y+self.radius } }
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
            pos: Vec2f { x: 400.0, y: 300.0 }, 
            radius: BALL_RADIUS, 
            velocity: Vec2f { x: 0.0, y: 0.0 } 
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
            player_type: PlayerType::BOT 
        }
    };

    while let Some( event) = window.next() {

        handle_input(&event, &mut game_state);
        handle_bot(&mut game_state);
        update_game(&mut game_state);

        window.draw_2d(&event, |context, g2d, _| {

            clear(BACKGROUND_COLOR, g2d);

            game_state.get_paddle_left().draw(&context, g2d);
            game_state.get_paddle_right().draw(&context, g2d);
            game_state.get_ball().draw(&context, g2d);

            draw_bbox(game_state.get_paddle_left(), &context, g2d);
            draw_bbox(game_state.get_paddle_right(), &context, g2d);
            draw_bbox(game_state.get_ball(), &context, g2d);
        });

    }
}

fn draw_bbox(element: &dyn GameElement, context: &Context, g2d: &mut G2d) {
    let bbox = element.get_bbox();
    rectangle([1.0, 0.0, 0.0, 0.2],
        [bbox.ll.x, bbox.ll.y, bbox.ur.x - bbox.ll.x, bbox.ur.y - bbox.ll.y],
        context.transform, g2d);
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

fn update_game(game_state: &mut GameState) {
    // move paddle
    game_state.get_paddle_left().pos.y += game_state.get_paddle_left().velocity.y;
    game_state.get_paddle_right().pos.y += game_state.get_paddle_right().velocity.y;

    // move ball
    // TODO
}