use piston_window::*;

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

struct Paddle {
    x: f64,
    y: f64,
    velocity_y: f64,
    player_type: PlayerType
}

impl Paddle {
    fn render(&self, context: &Context, g2d: &mut G2d) {
        rectangle(PADDLE_COLOR,
                  [self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT],
                  context.transform, g2d);
    }
    fn is_human(&self) -> bool {
        matches!(self.player_type, PlayerType::HUMAN)
    }
}

struct Ball {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Ball {
    fn render(&self, context: &Context, g2d: &mut G2d) {
        ellipse(BALL_COLOR,
                [self.x - BALL_RADIUS, self.y - BALL_RADIUS, BALL_RADIUS * 2.0, BALL_RADIUS * 2.0],
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
            self.paddle_left.velocity_y = velocity_y;
        }
        if self.paddle_right.is_human() {
            self.paddle_right.velocity_y = velocity_y;
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
        ball: Ball { x: 400.0, y: 300.0, velocity_x: 0.0, velocity_y: 0.0 },
        paddle_left: Paddle { x: 100.0, y: 250.0,  velocity_y: 0.0, player_type: PlayerType::HUMAN },
        paddle_right: Paddle { x: 700.0, y: 250.0,  velocity_y: 0.0, player_type: PlayerType::BOT }
    };

    while let Some( event) = window.next() {

        handle_input(&event, &mut game_state);
        handle_bot(&mut game_state);
        update_game(&mut game_state);

        window.draw_2d(&event, |context, g2d, _| {
            clear(BACKGROUND_COLOR, g2d);
            game_state.get_paddle_left().render(&context, g2d);
            game_state.get_paddle_right().render(&context, g2d);
            game_state.get_ball().render(&context, g2d);
        });

    }
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
    game_state.get_paddle_left().y += game_state.get_paddle_left().velocity_y;
    game_state.get_paddle_right().y += game_state.get_paddle_right().velocity_y;

    // move ball
    // TODO
}