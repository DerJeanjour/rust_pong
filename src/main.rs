use piston_window::*;
use rand::Rng;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_FPS: u64 = 60;

const FONT_SIZE: f64 = 24.0;
const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const FONT_PATH: &str = "assets/8bitOperatorPlus-Bold.ttf";

const BALL_RADIUS: f64 = 10.0;
const BALL_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BALL_VELOCITY_INC: f64 = 2.0;
const BALL_BOUNCE_VELOCITY_INC: f64 = 0.1;

const CONTROL_LEFT: ControlType = ControlType::PLAYER;
const CONTROL_RIGHT: ControlType = ControlType::BOT;

const PADDLE_WIDTH: f64 = 10.0;
const PADDLE_HEIGHT: f64 = 160.0;
const PADDLE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PADDLE_VELOCITY_INC: f64 = 1.8;
const PADDLE_VELOCITY_IMPACT_ON_BALL: f64 = 0.2;

const BOT_VIEW_DISTANCE: f64 = 300.0;

const BACKGROUND_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

const ZERO_VEC: Vec2f = Vec2f { x: 0.0, y: 0.0 };

#[derive(Clone)]
struct Vec2f {
    x: f64,
    y: f64,
}

impl Vec2f {
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    fn normalize(&self) -> Vec2f {
        let len = self.length();
        if len <= 0.0 {
            return ZERO_VEC.clone();
        }
        Vec2f {
            x: self.x / len,
            y: self.y / len,
        }
    }
    fn dot(&self, vec: &Vec2f) -> f64 {
        self.x * vec.x + self.y * vec.y
    }
    fn distance(&self, vec: &Vec2f) -> f64 {
        self.sub(&vec).length()
    }
    fn faces(&self, vec: &Vec2f) -> bool {
        self.dot(vec) < 0.0
    }
    fn mul(&self, factor: f64) -> Vec2f {
        Vec2f {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    fn sub(&self, vec: &Vec2f) -> Vec2f {
        Vec2f {
            x: self.x - vec.x,
            y: self.y - vec.y,
        }
    }
    fn add(&self, vec: &Vec2f) -> Vec2f {
        Vec2f {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}

impl ToString for Vec2f {
    fn to_string(&self) -> String {
        format!("({:.2}, {:.2})", self.x, self.y)
    }
}

struct BBox {
    min: Vec2f,
    max: Vec2f,
}

#[derive(Clone)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
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
enum ControlType {
    PLAYER,
    BOT,
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
    player_type: ControlType,
    dir: Direction,
}

impl GameElement for Paddle {
    fn get_bbox(&self) -> BBox {
        BBox {
            min: Vec2f {
                x: self.pos.x,
                y: self.pos.y,
            },
            max: Vec2f {
                x: self.pos.x + self.size.x,
                y: self.pos.y + self.size.y,
            },
        }
    }
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle(
            PADDLE_COLOR,
            [self.pos.x, self.pos.y, self.size.x, self.size.y],
            context.transform,
            g2d,
        );
    }
}

#[derive(Clone)]
struct Ball {
    pos: Vec2f,
    radius: f64,
    velocity: Vec2f,
}

impl GameElement for Ball {
    fn get_bbox(&self) -> BBox {
        BBox {
            min: Vec2f {
                x: self.pos.x - self.radius,
                y: self.pos.y - self.radius,
            },
            max: Vec2f {
                x: self.pos.x + self.radius,
                y: self.pos.y + self.radius,
            },
        }
    }
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        ellipse(
            BALL_COLOR,
            [
                self.pos.x - self.radius,
                self.pos.y - self.radius,
                self.radius * 2.0,
                self.radius * 2.0,
            ],
            context.transform,
            g2d,
        );
    }
}

struct GameState {
    ball: Ball,
    paddle_left: Paddle,
    paddle_right: Paddle,
    round: i32,
    round_bounces: i32,
    last_win: i32, // 0=no win / neg=left / pos=right
    score_left: i32,
    score_right: i32,
}

impl GameState {
    fn set_human_paddle_velocity_y(&mut self, velocity_y: f64) {
        if matches!(self.paddle_left.player_type, ControlType::PLAYER) {
            self.paddle_left.velocity.y = velocity_y;
        }
        if matches!(self.paddle_right.player_type, ControlType::PLAYER) {
            self.paddle_right.velocity.y = velocity_y;
        }
    }
}

fn main() {
    // init window
    let mut window: PistonWindow = WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(WINDOW_FPS);
    window.set_ups(WINDOW_FPS);
    let mut glyphs = Glyphs::new(
        FONT_PATH,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .unwrap();

    // init game state
    let mut game_state = GameState {
        ball: Ball {
            pos: ZERO_VEC,
            radius: BALL_RADIUS,
            velocity: ZERO_VEC,
        },
        paddle_left: Paddle {
            pos: Vec2f {
                x: WINDOW_WIDTH as f64 * 0.1,
                y: WINDOW_HEIGHT as f64 / 2.0,
            },
            size: Vec2f {
                x: PADDLE_WIDTH,
                y: PADDLE_HEIGHT,
            },
            velocity: Vec2f { x: 0.0, y: 0.0 },
            player_type: CONTROL_LEFT,
            dir: Direction::RIGHT,
        },
        paddle_right: Paddle {
            pos: Vec2f {
                x: WINDOW_WIDTH as f64 * 0.9,
                y: WINDOW_HEIGHT as f64 / 2.0,
            },
            size: Vec2f {
                x: PADDLE_WIDTH,
                y: PADDLE_HEIGHT,
            },
            velocity: Vec2f { x: 0.0, y: 0.0 },
            player_type: CONTROL_RIGHT,
            dir: Direction::LEFT,
        },
        round: 0,
        round_bounces: 0,
        last_win: 0,
        score_left: 0,
        score_right: 0,
    };
    new_round(&mut game_state);

    // render loop
    while let Some(event) = window.next() {
        // handle game logic
        handle_input(&event, &mut game_state);
        handle_bot(&mut game_state);
        let won_round = update_game(&mut game_state);
        if won_round != 0 {
            game_state.last_win = won_round;
            new_round(&mut game_state);
        }

        // render game
        window.draw_2d(&event, |context, g2d, device| {
            clear(BACKGROUND_COLOR, g2d);

            // draw game elements
            game_state.paddle_left.draw(&context, g2d);
            game_state.paddle_right.draw(&context, g2d);
            game_state.ball.draw(&context, g2d);

            // draw UI
            let round_count_pos = Vec2f {
                x: (WINDOW_WIDTH / 2) as f64,
                y: FONT_SIZE + 10.0,
            };
            draw_text(
                &format!("-{}-", game_state.round),
                &context,
                g2d,
                &mut glyphs,
                round_count_pos,
            );

            let bounce_count_pos = Vec2f {
                x: (WINDOW_WIDTH / 2) as f64,
                y: WINDOW_HEIGHT as f64 - 20.0,
            };
            draw_text(
                &format!("x{:.2}", get_rounce_bounce_factor(&game_state)),
                &context,
                g2d,
                &mut glyphs,
                bounce_count_pos,
            );

            let score_left_pos = Vec2f {
                x: 30.0,
                y: FONT_SIZE + 10.0,
            };
            draw_text(
                &game_state.score_left.to_string(),
                &context,
                g2d,
                &mut glyphs,
                score_left_pos,
            );

            let score_right_pos = Vec2f {
                x: WINDOW_WIDTH as f64 - 30.0,
                y: FONT_SIZE + 10.0,
            };
            draw_text(
                &game_state.score_right.to_string(),
                &context,
                g2d,
                &mut glyphs,
                score_right_pos,
            );

            glyphs.factory.encoder.flush(device);
        });
    }
}

fn draw_text(text: &str, context: &Context, g2d: &mut G2d, glyphs: &mut Glyphs, pos: Vec2f) {
    let x_offset = glyphs.width(FONT_SIZE as u32, &text).unwrap() / 2 as f64;
    text::Text::new_color(FONT_COLOR, FONT_SIZE as u32)
        .draw(
            text,
            glyphs,
            &context.draw_state,
            context.transform.trans(pos.x - x_offset, pos.y),
            g2d,
        )
        .unwrap();
}

fn new_round(game_state: &mut GameState) {
    // ball x direction
    let mut x_dir = -1.0;
    if game_state.last_win != 0 {
        x_dir = game_state.last_win as f64;
    }

    // ball y direction
    let mut rng = rand::thread_rng();
    let mut y_dir = -1.0;
    if rng.gen_range(0.0..1.0) >= 0.5 {
        y_dir = 1.0;
    }

    // update game states
    if game_state.last_win < 0 {
        game_state.score_left += 1;
    } else if game_state.last_win > 0 {
        game_state.score_right += 1;
    }
    game_state.round += 1;
    game_state.round_bounces = 0;
    game_state.ball.pos = Vec2f {
        x: (WINDOW_WIDTH / 2) as f64,
        y: (WINDOW_HEIGHT / 2) as f64,
    };
    game_state.ball.velocity = Vec2f {
        x: x_dir * BALL_VELOCITY_INC,
        y: rng.gen_range(0.2..0.8) * y_dir * BALL_VELOCITY_INC,
    };
}

fn handle_input(event: &Event, game_state: &mut GameState) {
    // on key press
    if let Some(Button::Keyboard(key)) = event.press_args() {
        match key {
            Key::Up => game_state.set_human_paddle_velocity_y(-PADDLE_VELOCITY_INC),
            Key::Down => game_state.set_human_paddle_velocity_y(PADDLE_VELOCITY_INC),
            Key::Space => {
                game_state.last_win = 0;
                new_round(game_state);
            }
            _ => {}
        }
    }

    // on key release
    if let Some(Button::Keyboard(key)) = event.release_args() {
        match key {
            Key::Up => game_state.set_human_paddle_velocity_y(0.0),
            Key::Down => game_state.set_human_paddle_velocity_y(0.0),
            _ => {}
        }
    }
}

fn handle_bot(game_state: &mut GameState) {
    // collect bot paddles
    let mut bot_paddles: Vec<&mut Paddle> = Vec::new();
    if matches!(game_state.paddle_left.player_type, ControlType::BOT) {
        bot_paddles.push(&mut game_state.paddle_left);
    }
    if matches!(game_state.paddle_right.player_type, ControlType::BOT) {
        bot_paddles.push(&mut game_state.paddle_right);
    }

    // update bot paddle velocities
    for bot in bot_paddles {
        let ball_x = Vec2f {
            x: game_state.ball.pos.x,
            y: 0.0,
        };
        let bot_x = Vec2f {
            x: bot.pos.x,
            y: 0.0,
        };
        let distance = ball_x.distance(&bot_x);

        // only move bot in y direction of ball if:
        // 1. ball is in view distance
        // 2. ball is moving towards paddle
        if distance < BOT_VIEW_DISTANCE && bot.dir.vector().faces(&game_state.ball.velocity) {
            let paddle_height_offset = bot.size.y / 2.0; // paddle height offset to center
            if game_state.ball.pos.y > bot.pos.y + paddle_height_offset {
                bot.velocity = Direction::DOWN.vector().mul(PADDLE_VELOCITY_INC);
            }
            if game_state.ball.pos.y < bot.pos.y {
                bot.velocity = Direction::UP.vector().mul(PADDLE_VELOCITY_INC);
            }
        } else {
            bot.velocity = ZERO_VEC;
        }
    }
}

fn update_game(game_state: &mut GameState) -> i32 {
    // immutable game elements
    let ball = game_state.ball.clone();
    let left_paddle = game_state.paddle_left.clone();
    let right_paddle = game_state.paddle_right.clone();

    // move paddle
    const MAX_H: f64 = WINDOW_HEIGHT as f64;
    let left_paddle_velocity = left_paddle.pos.y + left_paddle.velocity.y;
    let right_paddle_velocity = right_paddle.pos.y + right_paddle.velocity.y;
    // clamp to positions window bounds
    game_state.paddle_left.pos.y = left_paddle_velocity.clamp(0.0, MAX_H - left_paddle.size.y);
    game_state.paddle_right.pos.y = right_paddle_velocity.clamp(0.0, MAX_H - right_paddle.size.y);

    // handle ball out of bounds on width
    if is_out_of_bounds_on_width(&ball.get_bbox()) {
        // early break, signal new round
        if Direction::LEFT.vector().faces(&ball.velocity) {
            return -1; // left won round
        }
        return 1; // right won round
    }

    let mut bounce_direction = ball.velocity.normalize();

    // handle ball out of bounds on height
    if is_out_of_bounds_on_height(&ball.get_bbox()) {
        bounce_direction.y *= -1.0;
    }

    // handle paddle collisions
    for paddle in [&game_state.paddle_left, &game_state.paddle_right] {
        if has_collision(&ball.get_bbox(), &paddle.get_bbox()) {
            // get bounce direction
            bounce_direction = reflect(&ball.velocity, &paddle.dir.vector());
            // add player velocity impact
            bounce_direction =
                bounce_direction.add(&paddle.velocity.mul(PADDLE_VELOCITY_IMPACT_ON_BALL));
            game_state.round_bounces += 1;
        }
    }

    // update ball velocity
    bounce_direction = bounce_direction.normalize();
    let round_bounce_factor = get_rounce_bounce_factor(&game_state);
    let velocity_factor = BALL_VELOCITY_INC * round_bounce_factor;
    game_state.ball.velocity.x = bounce_direction.x * velocity_factor;
    game_state.ball.velocity.y = bounce_direction.y * velocity_factor;

    // move ball
    game_state.ball.pos.x += game_state.ball.velocity.x;
    game_state.ball.pos.y += game_state.ball.velocity.y;

    0
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
    bbox_a.min.x <= bbox_b.max.x
        && bbox_a.max.x >= bbox_b.min.x
        && bbox_a.min.y <= bbox_b.max.y
        && bbox_a.max.y >= bbox_b.min.y
}

fn reflect(dir: &Vec2f, normal: &Vec2f) -> Vec2f {
    if !normal.faces(&dir) {
        // if direction is the same, dont reflect
        return dir.normalize();
    }

    let dot_product = dir.dot(normal);
    Vec2f {
        x: dir.x - 2.0 * dot_product * normal.x,
        y: dir.y - 2.0 * dot_product * normal.y,
    }
    .normalize()
}

fn get_rounce_bounce_factor(game_state: &GameState) -> f64 {
    game_state.round_bounces as f64 * BALL_BOUNCE_VELOCITY_INC + 1.0
}
